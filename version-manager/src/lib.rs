pub mod cli;
pub mod files;
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
    RegexError(regex::Error),
    IncompleteCommand,
    InvalidOperation,
    NoCommand,
    NoValue,
    NoNegatives,
}

pub type VersionResult<T> = Result<T, VersionError>;

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
            VersionError::RegexError(e) => format!("Regex Error: {}", e),
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
            VersionError::RegexError(_) => ErrorKind::ValueValidation,
            VersionError::IncompleteCommand => ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::InvalidOperation => ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::NoCommand => ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::NoValue => ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::NoNegatives => ErrorKind::InvalidValue,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use clap::CommandFactory;
    #[test]
    fn no_cmd() {
        let error = VersionError::NoCommand;
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(
            Into::<ErrorKind>::into(&error),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }
    #[test]
    fn io_error() {
        let error =
            VersionError::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(Into::<ErrorKind>::into(&error), ErrorKind::Io);
    }
    #[test]
    fn regex_error() {
        let error = VersionError::RegexError(regex::Error::Syntax("test".to_string()));
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(Into::<ErrorKind>::into(&error), ErrorKind::ValueValidation);
    }
    #[test]
    fn toml_de_error() {
        use serde::de::Error;
        let error = VersionError::TomlDeError(toml::de::Error::missing_field("test"));
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(Into::<ErrorKind>::into(&error), ErrorKind::Io);
    }
    #[test]
    fn toml_ser_error() {
        use serde::ser::Error;
        let error = VersionError::TomlSerError(toml::ser::Error::custom("test"));
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(Into::<ErrorKind>::into(&error), ErrorKind::Io);
    }
    #[test]
    fn incomplete_command_error() {
        let error = VersionError::IncompleteCommand;
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(
            Into::<ErrorKind>::into(&error),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }
    #[test]
    fn invalid_operation_error() {
        let error = VersionError::InvalidOperation;
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(
            Into::<ErrorKind>::into(&error),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }
    #[test]
    fn no_value_error() {
        let error = VersionError::NoValue;
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(
            Into::<ErrorKind>::into(&error),
            ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }
    #[test]
    fn no_negatives_error() {
        let error = VersionError::NoNegatives;
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(Into::<ErrorKind>::into(&error), ErrorKind::InvalidValue);
    }
}
