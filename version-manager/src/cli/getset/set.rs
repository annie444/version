use crate::{
    version::{run, Operator, SetTypes, VersionFile},
    CommandRun, VersionError, VersionResult,
};
use clap::{builder::RangedU64ValueParser, Args, Subcommand};

#[derive(Args, Debug, Clone)]
/// Set the version number
pub struct Set {
    #[arg(value_parser = RangedU64ValueParser::<u8>::new(), exclusive = true)]
    /// The value to set the version number to
    pub value: Option<u8>,
    #[command(subcommand)]
    /// Increment or decrement the version number by 1
    pub command: Option<UpDown>,
}

impl CommandRun for Set {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        if let Some(value) = &self.value {
            version.operator = Some(Operator::Set(SetTypes::Number(*value)));
            run(version)
        } else if let Some(command) = &self.command {
            command.run(version)
        } else {
            Err(VersionError::NoValue)
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
#[command(rename_all = "lower", arg_required_else_help(true))]
pub enum UpDown {
    #[command(name = "+")]
    /// Increment the version number by 1
    Up,
    #[command(name = "-")]
    /// Decrement the version number by 1
    Down,
}

impl CommandRun for UpDown {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        match self {
            UpDown::Up => {
                version.operator = Some(Operator::Set(SetTypes::AddNumber));
                run(version)
            }
            UpDown::Down => {
                version.operator = Some(Operator::Set(SetTypes::SubNumber));
                run(version)
            }
        }
    }
}
