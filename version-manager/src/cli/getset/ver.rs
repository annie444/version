use crate::{VersionError, version::Operator};
use clap::{Parser, builder::NonEmptyStringValueParser};

#[derive(Parser, Debug, Clone, PartialEq)]
/// Get or set the build version
#[command(arg_required_else_help(true))]
pub struct SetVer {
    #[arg(value_parser = NonEmptyStringValueParser::new())]
    pub val: String,
}

impl TryFrom<SetVer> for Operator {
    type Error = VersionError;

    fn try_from(cmd: SetVer) -> Result<Self, Self::Error> {
        Ok(Operator::SetVersion(cmd.val))
    }
}

impl TryFrom<&SetVer> for Operator {
    type Error = VersionError;

    fn try_from(cmd: &SetVer) -> Result<Self, Self::Error> {
        Ok(Operator::SetVersion(cmd.val.clone()))
    }
}
