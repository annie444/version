use super::Set;
use crate::{
    version::{run, Operator, VersionFile},
    CommandRun, VersionResult,
};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
/// Get or set the version number
#[command(arg_required_else_help(true))]
pub struct GetSetRm {
    #[command(subcommand)]
    pub command: GetSetRmCommand,
}

impl CommandRun for GetSetRm {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        self.command.run(version)
    }
}

#[derive(Subcommand, Debug, Clone)]
/// Get or set the version number
pub enum GetSetRmCommand {
    /// Print the current version
    Get,
    /// Set the version number
    Set(Set),
    /// Remove the version identifier
    Rm,
}

impl CommandRun for GetSetRmCommand {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        match self {
            GetSetRmCommand::Get => {
                version.operator = Some(Operator::Get);
                run(version)
            }
            GetSetRmCommand::Set(set) => set.run(version),
            GetSetRmCommand::Rm => {
                version.operator = Some(Operator::Rm);
                run(version)
            }
        }
    }
}
