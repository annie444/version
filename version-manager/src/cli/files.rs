use crate::{
    VersionError,
    files::TrackedFiles,
    version::{Operator, SetTypes},
};
use clap::{Args, Parser, Subcommand, value_parser};
use clio::ClioPath;
use regex::Regex;

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(arg_required_else_help(true))]
pub struct FilesCommand {
    #[clap(subcommand)]
    pub command: Files,
}

impl TryFrom<FilesCommand> for Operator {
    type Error = VersionError;

    fn try_from(cmd: FilesCommand) -> Result<Self, Self::Error> {
        cmd.command.try_into()
    }
}

impl TryFrom<&FilesCommand> for Operator {
    type Error = VersionError;

    fn try_from(cmd: &FilesCommand) -> Result<Self, Self::Error> {
        (&cmd.command).try_into()
    }
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
#[command(arg_required_else_help(true))]
pub enum Files {
    /// Add a file to add the version number
    Track(TrackFile),
    /// Remove a file from tracking the version number
    Rm(File),
    /// Set the version number from a file
    Update(File),
    /// Update all files
    UpdateAll,
    /// List tracked files
    List,
}

impl TryFrom<Files> for Operator {
    type Error = VersionError;

    fn try_from(cmd: Files) -> Result<Self, Self::Error> {
        let op = match cmd {
            Files::Track(track) => Operator::AddFile(track.try_into()?),
            Files::Rm(file) => Operator::RmFile(file.to_string()),
            Files::Update(file) => Operator::Update(file.to_string()),
            Files::UpdateAll => Operator::UpdateAll,
            Files::List => Operator::ListFiles,
        };
        Ok(op)
    }
}

impl TryFrom<&Files> for Operator {
    type Error = VersionError;

    fn try_from(cmd: &Files) -> Result<Self, Self::Error> {
        let op = match cmd {
            Files::Track(track) => Operator::AddFile(track.try_into()?),
            Files::Rm(file) => Operator::RmFile(file.to_string()),
            Files::Update(file) => Operator::Update(file.to_string()),
            Files::UpdateAll => Operator::UpdateAll,
            Files::List => Operator::ListFiles,
        };
        Ok(op)
    }
}

#[derive(Args, Debug, Clone, PartialEq)]
#[command(arg_required_else_help(true))]
pub struct TrackFile {
    /// The path to the file to track
    #[arg(value_parser = value_parser!(ClioPath).exists().is_file())]
    pub path: ClioPath,
    /// The expression to match the version number
    ///
    /// This expression should be a regex with a single capture group that matches the version number
    pub expr: String,
}

impl TryFrom<&TrackFile> for SetTypes {
    type Error = VersionError;

    fn try_from(track_file: &TrackFile) -> Result<Self, Self::Error> {
        let track_file = TrackedFiles::new_from_path_and_regex(
            track_file.path.to_path_buf(),
            track_file.expr.clone().parse::<Regex>()?,
        );
        Ok(SetTypes::NewFile(track_file))
    }
}

impl TryFrom<TrackFile> for SetTypes {
    type Error = VersionError;

    fn try_from(track_file: TrackFile) -> Result<Self, Self::Error> {
        let track_file = TrackedFiles::new_from_path_and_regex(
            track_file.path.to_path_buf(),
            track_file.expr.clone().parse::<Regex>()?,
        );
        Ok(SetTypes::NewFile(track_file))
    }
}

#[derive(Args, Debug, Clone, PartialEq)]
#[command(arg_required_else_help(true))]
pub struct File {
    /// The path to the file
    #[arg(value_parser = clap::value_parser!(ClioPath).exists().is_file())]
    pub path: ClioPath,
}

impl ToString for File {
    fn to_string(&self) -> String {
        self.path.to_string()
    }
}

impl TryFrom<&File> for SetTypes {
    type Error = VersionError;

    fn try_from(file: &File) -> Result<Self, Self::Error> {
        Ok(SetTypes::String(file.path.to_string()))
    }
}

impl TryFrom<File> for SetTypes {
    type Error = VersionError;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        Ok(SetTypes::String(file.path.to_string()))
    }
}
