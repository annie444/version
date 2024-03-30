pub mod cli;
pub mod version;
use clap::{
    error::{Error, ErrorKind},
    Command,
};
use core::fmt::Display;

#[derive(Debug)]
pub enum VersionError {
    IoError(std::io::Error),
    TomlDeError(toml::de::Error),
    TomlSerError(toml::ser::Error),
    IncompleteCommand,
    InvalidOperation,
    NoCommand,
    NoValue,
    NoNegatives,
}

impl Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl VersionError {
    pub fn to_string(&self) -> String {
        match self {
            VersionError::IoError(e) => format!("IO Error: {}", e),
            VersionError::TomlDeError(e) => format!("TOML Deserialize Error: {}", e),
            VersionError::TomlSerError(e) => format!("TOML Serialize Error: {}", e),
            VersionError::IncompleteCommand => "Please specify a subcommand".to_string(),
            VersionError::InvalidOperation => "Invalid operation".to_string(),
            VersionError::NoCommand => "Unable to parse the command".to_string(),
            VersionError::NoValue => "No value given".to_string(),
            VersionError::NoNegatives => "Negative values are not allowed".to_string(),
        }
    }

    pub fn error(&self, cmd: &mut Command) -> Error {
        cmd.error(Into::<ErrorKind>::into(self), self.to_string())
    }

    pub fn terminate(&self, cmd: &mut Command) -> ! {
        let err = self.error(cmd);
        err.exit()
    }
}

impl Into<ErrorKind> for &VersionError {
    fn into(self) -> ErrorKind {
        match self {
            VersionError::IoError(_) => ErrorKind::Io,
            VersionError::TomlDeError(_) => ErrorKind::Io,
            VersionError::TomlSerError(_) => ErrorKind::Io,
            VersionError::IncompleteCommand => ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::InvalidOperation => ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::NoCommand => ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::NoValue => ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::NoNegatives => ErrorKind::InvalidValue,
        }
    }
}
