use super::Set;
use crate::{
    version::{Operator, VersionFile},
    VersionError,
};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
/// Get or set the version number
#[command(arg_required_else_help(true))]
pub struct GetSetRm {
    #[command(subcommand)]
    pub command: GetSetRmCommand,
}

impl GetSetRm {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
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

impl GetSetRmCommand {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        match self {
            GetSetRmCommand::Get => {
                version.operator = Some(Operator::Get);
                version.run()
            }
            GetSetRmCommand::Set(set) => set.run(version),
            GetSetRmCommand::Rm => {
                version.operator = Some(Operator::Rm);
                version.run()
            }
        }
    }
}
