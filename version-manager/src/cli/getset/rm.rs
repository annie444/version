use super::Set;
use crate::{VersionError, version::Operator};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone, PartialEq)]
/// Get or set the version number
#[command(arg_required_else_help(true))]
pub struct GetSetRm {
    #[command(subcommand)]
    pub command: GetSetRmCommand,
}

impl TryFrom<GetSetRm> for Operator {
    type Error = VersionError;

    fn try_from(cmd: GetSetRm) -> Result<Self, Self::Error> {
        cmd.command.try_into()
    }
}

impl TryFrom<&GetSetRm> for Operator {
    type Error = VersionError;

    fn try_from(cmd: &GetSetRm) -> Result<Self, Self::Error> {
        (&cmd.command).try_into()
    }
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
/// Get or set the version number
pub enum GetSetRmCommand {
    /// Print the current version
    Get,
    /// Set the version number
    Set(Set),
    /// Remove the version identifier
    Rm,
    /// Reset the subversions
    Reset,
}

impl TryFrom<&GetSetRmCommand> for Operator {
    type Error = VersionError;

    fn try_from(cmd: &GetSetRmCommand) -> Result<Self, Self::Error> {
        match cmd {
            GetSetRmCommand::Get => Ok(Operator::Get),
            GetSetRmCommand::Set(set) => Ok(Operator::Set(set.try_into()?)),
            GetSetRmCommand::Rm => Ok(Operator::Rm),
            GetSetRmCommand::Reset => Ok(Operator::Reset),
        }
    }
}

impl TryFrom<GetSetRmCommand> for Operator {
    type Error = VersionError;

    fn try_from(cmd: GetSetRmCommand) -> Result<Self, Self::Error> {
        match cmd {
            GetSetRmCommand::Get => Ok(Operator::Get),
            GetSetRmCommand::Set(set) => Ok(Operator::Set(set.try_into()?)),
            GetSetRmCommand::Rm => Ok(Operator::Rm),
            GetSetRmCommand::Reset => Ok(Operator::Reset),
        }
    }
}
