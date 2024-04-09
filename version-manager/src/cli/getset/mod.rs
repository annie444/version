pub mod build;
pub mod rm;
pub mod set;

pub use build::GetSetBuild;
pub use rm::GetSetRm;
pub use set::Set;

use crate::{
    version::{run, Operator, VersionFile},
    CommandRun, VersionResult,
};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
/// Get or set the version number
#[command(arg_required_else_help(true))]
pub struct GetSet {
    #[command(subcommand)]
    pub command: GetSetCommand,
}

impl CommandRun for GetSet {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        self.command.run(version)
    }
}

#[derive(Subcommand, Debug, Clone)]
/// Get or set the version number
pub enum GetSetCommand {
    /// Print the current version
    Get,
    /// Set the version number
    Set(Set),
    /// Reset the subversions
    Reset,
}

impl CommandRun for GetSetCommand {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        match self {
            GetSetCommand::Get => {
                version.operator = Some(Operator::Get);
                run(version)
            }
            GetSetCommand::Set(set) => set.run(version),
            GetSetCommand::Reset => {
                version.operator = Some(Operator::Reset);
                run(version)
            }
        }
    }
}
