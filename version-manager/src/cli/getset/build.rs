use crate::{
    version::{run, Operator, SetTypes, VersionFile},
    CommandRun, VersionResult,
};
use clap::{builder::NonEmptyStringValueParser, Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
/// Get or set the build version
#[command(arg_required_else_help(true))]
pub struct GetSetBuild {
    #[command(subcommand)]
    pub command: GetSetBuildCommand,
}

impl CommandRun for GetSetBuild {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
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

impl CommandRun for GetSetBuildCommand {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        match self {
            GetSetBuildCommand::Get => {
                version.operator = Some(Operator::Get);
                run(version)
            }
            GetSetBuildCommand::Set(set) => set.run(version),
            GetSetBuildCommand::Rm => {
                version.operator = Some(Operator::Rm);
                run(version)
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

impl CommandRun for SetBuild {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        version.operator = Some(Operator::Set(SetTypes::String(self.value.clone())));
        run(version)
    }
}
