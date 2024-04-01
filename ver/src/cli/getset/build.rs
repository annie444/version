use crate::{
    version::{Operator, SetTypes, VersionFile},
    VersionError,
};
use clap::{builder::NonEmptyStringValueParser, Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
/// Get or set the build version
#[command(arg_required_else_help(true))]
pub struct GetSetBuild {
    #[command(subcommand)]
    pub command: GetSetBuildCommand,
}

impl GetSetBuild {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        self.command.run(version)
    }
}

#[derive(Subcommand, Debug, Clone)]
/// Get or set the build version
pub enum GetSetBuildCommand {
    Get,
    Set(SetBuild),
    Rm,
}

impl GetSetBuildCommand {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        match self {
            GetSetBuildCommand::Get => {
                version.operator = Some(Operator::Get);
                version.run()
            }
            GetSetBuildCommand::Set(set) => set.run(version),
            GetSetBuildCommand::Rm => {
                version.operator = Some(Operator::Rm);
                version.run()
            }
        }
    }
}

#[derive(Args, Debug, Clone)]
/// Set the build version
pub struct SetBuild {
    #[arg(value_parser = NonEmptyStringValueParser::new())]
    pub value: String,
}

impl SetBuild {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        version.operator = Some(Operator::Set(SetTypes::String(self.value.clone())));
        version.run()
    }
}
