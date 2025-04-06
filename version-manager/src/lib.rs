pub mod cli;
pub mod files;
pub mod run;
pub mod version;

use clap::{Command, error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VersionError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("TOML Deserialize Error: {0}")]
    TomlDeError(#[from] toml::de::Error),
    #[error("TOML Serialize Error: {0}")]
    TomlSerError(#[from] toml::ser::Error),
    #[error("Regex Error: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Incomplete Command")]
    IncompleteCommand,
    #[error("Invalid Operation")]
    InvalidOperation,
    #[error("No Command specified")]
    NoCommand,
    #[error("No Value specified")]
    NoValue,
    #[error("No Negatives allowed")]
    NoNegatives,
    #[error("Invalid Command")]
    InvalidCommand,
    #[error("Empty Version")]
    EmptyVersion,
    #[error("Invalid Prerelease: {0}")]
    InvalidPrerelease(String),
    #[error("Invalid Version: {0}")]
    InvalidVersion(#[from] semver::Error),
    #[error("Package name required")]
    PackageNameRequired,
}

pub type VersionResult<T> = Result<T, VersionError>;

impl VersionError {
    pub fn cmd_error(&self, cmd: &mut Command) -> error::Error {
        cmd.error(Into::<error::ErrorKind>::into(self), self.to_string())
    }

    pub fn terminate(&self, cmd: &mut Command) -> ! {
        let err = self.cmd_error(cmd);
        err.exit()
    }
}

impl From<&VersionError> for error::ErrorKind {
    fn from(err: &VersionError) -> error::ErrorKind {
        match err {
            VersionError::IoError(_) => error::ErrorKind::Io,
            VersionError::TomlDeError(_) => error::ErrorKind::Io,
            VersionError::TomlSerError(_) => error::ErrorKind::Io,
            VersionError::RegexError(_) => error::ErrorKind::ValueValidation,
            VersionError::IncompleteCommand => {
                error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            }
            VersionError::InvalidOperation => {
                error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            }
            VersionError::NoCommand => error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::NoValue => error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
            VersionError::NoNegatives => error::ErrorKind::InvalidValue,
            VersionError::InvalidCommand => error::ErrorKind::InvalidValue,
            VersionError::EmptyVersion => error::ErrorKind::InvalidValue,
            VersionError::InvalidPrerelease(_) => error::ErrorKind::InvalidValue,
            VersionError::InvalidVersion(_) => error::ErrorKind::InvalidValue,
            VersionError::PackageNameRequired => {
                error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use clap::CommandFactory;
    use clap::error::ErrorKind;
    #[test]
    fn no_cmd() {
        let error = VersionError::NoCommand;
        let displ = error.to_string();
        assert!(displ.len() > 0);
        let cmd = cli::Cli::command();
        let err = error.cmd_error(&mut cmd.clone());
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
        let err = error.cmd_error(&mut cmd.clone());
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
        let err = error.cmd_error(&mut cmd.clone());
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
        let err = error.cmd_error(&mut cmd.clone());
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
        let err = error.cmd_error(&mut cmd.clone());
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
        let err = error.cmd_error(&mut cmd.clone());
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
        let err = error.cmd_error(&mut cmd.clone());
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
        let err = error.cmd_error(&mut cmd.clone());
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
        let err = error.cmd_error(&mut cmd.clone());
        let render = format!("{}", err.render());
        assert!(render.len() > 0);
        assert_eq!(Into::<ErrorKind>::into(&error), ErrorKind::InvalidValue);
    }
}
