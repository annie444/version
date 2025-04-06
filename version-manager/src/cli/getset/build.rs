use crate::{
    VersionError,
    version::{Operator, SetTypes},
};
use clap::{Args, Parser, Subcommand, builder::NonEmptyStringValueParser};

#[derive(Parser, Debug, Clone, PartialEq)]
/// Get or set the build version
#[command(arg_required_else_help(true))]
pub struct GetSetBuild {
    #[command(subcommand)]
    pub command: GetSetBuildCommand,
}

impl TryFrom<GetSetBuild> for Operator {
    type Error = VersionError;

    fn try_from(cmd: GetSetBuild) -> Result<Self, Self::Error> {
        cmd.command.try_into()
    }
}

impl TryFrom<&GetSetBuild> for Operator {
    type Error = VersionError;

    fn try_from(cmd: &GetSetBuild) -> Result<Self, Self::Error> {
        (&cmd.command).try_into()
    }
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
/// Get or set the build version
pub enum GetSetBuildCommand {
    Get,
    Set(SetBuild),
    Rm,
}

impl TryFrom<&GetSetBuildCommand> for Operator {
    type Error = VersionError;

    fn try_from(cmd: &GetSetBuildCommand) -> Result<Self, Self::Error> {
        let op = match cmd {
            GetSetBuildCommand::Get => Operator::Get,
            GetSetBuildCommand::Set(set) => Operator::Set(set.try_into()?),
            GetSetBuildCommand::Rm => Operator::Rm,
        };
        Ok(op)
    }
}

impl TryFrom<GetSetBuildCommand> for Operator {
    type Error = VersionError;

    fn try_from(cmd: GetSetBuildCommand) -> Result<Self, Self::Error> {
        let op = match cmd {
            GetSetBuildCommand::Get => Operator::Get,
            GetSetBuildCommand::Set(set) => Operator::Set(set.try_into()?),
            GetSetBuildCommand::Rm => Operator::Rm,
        };
        Ok(op)
    }
}

#[derive(Args, Debug, Clone, PartialEq)]
/// Set the build version
pub struct SetBuild {
    #[arg(value_parser = NonEmptyStringValueParser::new())]
    pub value: String,
}

impl TryFrom<SetBuild> for SetTypes {
    type Error = VersionError;

    fn try_from(set: SetBuild) -> Result<Self, Self::Error> {
        Ok(SetTypes::String(set.value))
    }
}

impl TryFrom<&SetBuild> for SetTypes {
    type Error = VersionError;

    fn try_from(set: &SetBuild) -> Result<Self, Self::Error> {
        Ok(SetTypes::String(set.value.clone()))
    }
}
