use super::VersionCommand;
use crate::{version::VersionFile, VersionError};
use clap::{value_parser, Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use std::io;

#[derive(Parser, Debug, Clone)]
#[command(arg_required_else_help(true))]
/// A tool for managing the version of a project
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<VersionCommand>,
    #[arg(value_parser = value_parser!(Shell), exclusive = true)]
    /// Generate shell completions
    pub generator: Option<Shell>,
}

impl Cli {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        if let Some(generator) = self.generator {
            let mut cmd = Cli::command();
            Self::print_completions(generator, &mut cmd)
        } else if let Some(command) = &self.command {
            command.run(version)
        } else {
            Err(VersionError::NoCommand)
        }
    }

    fn print_completions<G: Generator>(gen: G, cmd: &mut Command) -> Result<(), VersionError> {
        generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
        Ok(())
    }
}
