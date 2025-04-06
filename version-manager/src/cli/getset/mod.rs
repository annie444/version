pub mod build;
pub mod rm;
pub mod set;
pub mod ver;

pub use build::GetSetBuild;
pub use rm::GetSetRm;
pub use set::Set;
pub use ver::SetVer;

use crate::{VersionError, version::Operator};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone, PartialEq)]
/// Get or set the version number
#[command(arg_required_else_help(true))]
pub struct GetSet {
    #[command(subcommand)]
    pub command: GetSetCommand,
}

impl TryFrom<GetSet> for Operator {
    type Error = VersionError;

    fn try_from(cmd: GetSet) -> Result<Self, Self::Error> {
        cmd.command.try_into()
    }
}

impl TryFrom<&GetSet> for Operator {
    type Error = VersionError;

    fn try_from(cmd: &GetSet) -> Result<Self, Self::Error> {
        (&cmd.command).try_into()
    }
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
/// Get or set the version number
pub enum GetSetCommand {
    /// Print the current version
    Get,
    /// Set the version number
    Set(Set),
    /// Reset the subversions
    Reset,
}

impl TryFrom<&GetSetCommand> for Operator {
    type Error = VersionError;

    fn try_from(cmd: &GetSetCommand) -> Result<Self, Self::Error> {
        match cmd {
            GetSetCommand::Get => Ok(Operator::Get),
            GetSetCommand::Set(set) => Ok(Operator::Set(set.try_into()?)),
            GetSetCommand::Reset => Ok(Operator::Reset),
        }
    }
}

impl TryFrom<GetSetCommand> for Operator {
    type Error = VersionError;

    fn try_from(cmd: GetSetCommand) -> Result<Self, Self::Error> {
        match cmd {
            GetSetCommand::Get => Ok(Operator::Get),
            GetSetCommand::Set(set) => Ok(Operator::Set(set.try_into()?)),
            GetSetCommand::Reset => Ok(Operator::Reset),
        }
    }
}
