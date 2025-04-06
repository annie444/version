use super::VersionCommand;
use crate::{VersionError, VersionResult, version::Scope};
use clap::{
    Command, CommandFactory, Parser,
    builder::{Styles, styling::AnsiColor},
    value_parser,
};
use clap_complete::{Generator, Shell, generate};
use std::io;

const STYLE: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .error(AnsiColor::Red.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::BrightBlue.on_default())
    .placeholder(AnsiColor::Blue.on_default())
    .valid(AnsiColor::Cyan.on_default())
    .invalid(AnsiColor::Magenta.on_default());

#[derive(Parser, Debug, Clone)]
#[command(arg_required_else_help(true), styles = STYLE, name = "version")]
/// A tool for managing the version of a project
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<VersionCommand>,
    #[arg(long, value_parser = value_parser!(Shell), exclusive = true)]
    /// Generate shell completions
    pub generator: Option<Shell>,
}

impl Cli {
    fn print_completions<G: Generator>(generator: G, cmd: &mut Command) -> VersionResult<()> {
        generate(
            generator,
            cmd,
            cmd.get_name().to_string(),
            &mut io::stdout(),
        );
        Ok(())
    }

    pub fn run(&mut self) -> VersionResult<Option<Scope>> {
        if let Some(generator) = self.generator.take() {
            let mut cmd = Cli::command();
            cmd.set_bin_name("version");
            Self::print_completions(generator, &mut cmd)?;
            return Ok(None);
        } else if let Some(command) = self.command.take() {
            return Ok(Some(command.try_into()?));
        } else {
            return Err(VersionError::InvalidOperation);
        }
    }
}

impl TryFrom<Cli> for Scope {
    type Error = VersionError;

    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        match cli.command {
            Some(command) => command.try_into(),
            None => Err(VersionError::InvalidOperation),
        }
    }
}
