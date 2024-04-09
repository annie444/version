use super::VersionCommand;
use crate::{version::VersionFile, CommandRun, VersionError, VersionResult};
use clap::{
    builder::{styling::AnsiColor, Styles},
    value_parser, Command, CommandFactory, Parser,
};
use clap_complete::{generate, Generator, Shell};
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
    fn print_completions<G: Generator>(gen: G, cmd: &mut Command) -> VersionResult<()> {
        generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
        Ok(())
    }
}

impl CommandRun for Cli {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        if let Some(generator) = self.generator {
            let mut cmd = Cli::command();
            Self::print_completions(generator, &mut cmd)
        } else if let Some(command) = &self.command {
            command.run(version)
        } else {
            Err(VersionError::NoCommand)
        }
    }
}
