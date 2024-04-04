use crate::{
    files::TrackedFiles,
    version::{Operator, SetTypes, VersionFile},
    VersionError,
};
use clap::{value_parser, Args, Parser, Subcommand};
use clio::ClioPath;
use regex::Regex;

#[derive(Parser, Debug, Clone)]
#[command(arg_required_else_help(true))]
pub struct FilesCommand {
    #[clap(subcommand)]
    pub command: Files,
}

#[derive(Subcommand, Debug, Clone)]
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

#[derive(Args, Debug, Clone)]
#[command(arg_required_else_help(true))]
pub struct TrackFile {
    /// The path to the file to track
    #[arg(value_parser = value_parser!(ClioPath).exists().is_file())]
    pub path: ClioPath,
    /// The expression to match the version number
    ///
    /// This expression should be a regex with a single capture group that matches the version number
    #[arg(value_parser = value_parser!(Regex))]
    pub expr: Regex,
}

#[derive(Args, Debug, Clone)]
#[command(arg_required_else_help(true))]
pub struct File {
    /// The path to the file
    #[arg(value_parser = clap::value_parser!(ClioPath).exists().is_file())]
    pub path: ClioPath,
}

impl FilesCommand {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        self.command.run(version)
    }
}

impl TrackFile {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        let track_file =
            TrackedFiles::new_from_path_and_regex(self.path.to_path_buf(), self.expr.clone());
        version.value = Some(SetTypes::NewFile(track_file));
        version.run()
    }
}

impl File {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        version.value = Some(SetTypes::String(self.path.to_string()));
        version.run()
    }
}

impl Files {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        match self {
            Files::Track(track) => {
                version.operator = Some(Operator::AddFile);
                track.run(version)
            }
            Files::Rm(file) => {
                version.operator = Some(Operator::RmFile);
                file.run(version)
            }
            Files::Update(file) => {
                version.operator = Some(Operator::Update);
                file.run(version)
            }
            Files::UpdateAll => {
                version.operator = Some(Operator::UpdateAll);
                version.run()
            }
            Files::List => {
                version.operator = Some(Operator::ListFiles);
                version.run()
            }
        }
    }
}
