use crate::{VersionError, version::SetTypes};
use clap::{Args, Subcommand, builder::RangedU64ValueParser};

#[derive(Args, Debug, Clone, PartialEq)]
/// Set the version number
pub struct Set {
    #[arg(value_parser = RangedU64ValueParser::<u64>::new(), exclusive = true)]
    /// The value to set the version number to
    pub value: Option<u64>,
    #[command(subcommand)]
    /// Increment or decrement the version number by 1
    pub command: Option<UpDown>,
}

impl TryFrom<&Set> for SetTypes {
    type Error = VersionError;

    fn try_from(set: &Set) -> Result<Self, VersionError> {
        if let Some(value) = &set.value {
            return Ok(SetTypes::Number(*value));
        } else if let Some(command) = &set.command {
            return Ok(command.try_into()?);
        } else {
            Err(VersionError::NoValue)
        }
    }
}

impl TryFrom<Set> for SetTypes {
    type Error = VersionError;

    fn try_from(set: Set) -> Result<Self, VersionError> {
        if let Some(value) = &set.value {
            return Ok(SetTypes::Number(*value));
        } else if let Some(command) = &set.command {
            return Ok(command.try_into()?);
        } else {
            Err(VersionError::NoValue)
        }
    }
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
#[command(rename_all = "lower", arg_required_else_help(true))]
pub enum UpDown {
    #[command(name = "+")]
    /// Increment the version number by 1
    Plus,
    #[command(name = "-")]
    /// Decrement the version number by 1
    Minus,
    /// Increment the version number by 1
    Up,
    /// Decrement the version number by 1
    Down,
}

impl TryFrom<&UpDown> for SetTypes {
    type Error = VersionError;

    fn try_from(updown: &UpDown) -> Result<Self, VersionError> {
        match updown {
            UpDown::Up => Ok(SetTypes::AddNumber),
            UpDown::Down => Ok(SetTypes::SubNumber),
            UpDown::Plus => Ok(SetTypes::AddNumber),
            UpDown::Minus => Ok(SetTypes::SubNumber),
        }
    }
}

impl TryFrom<UpDown> for SetTypes {
    type Error = VersionError;

    fn try_from(updown: UpDown) -> Result<Self, VersionError> {
        match updown {
            UpDown::Up => Ok(SetTypes::AddNumber),
            UpDown::Down => Ok(SetTypes::SubNumber),
            UpDown::Plus => Ok(SetTypes::AddNumber),
            UpDown::Minus => Ok(SetTypes::SubNumber),
        }
    }
}
