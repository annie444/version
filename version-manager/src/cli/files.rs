use crate::{
    files::TrackedFiles,
    version::{run, Operator, SetTypes, VersionFile},
    CommandRun, VersionResult,
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

impl CommandRun for FilesCommand {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        self.command.run(version)
    }
}

impl CommandRun for TrackFile {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        let track_file =
            TrackedFiles::new_from_path_and_regex(self.path.to_path_buf(), self.expr.clone());
        version.value = Some(SetTypes::NewFile(track_file));
        run(version)
    }
}

impl CommandRun for File {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
        version.value = Some(SetTypes::String(self.path.to_string()));
        run(version)
    }
}

impl CommandRun for Files {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
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
                run(version)
            }
            Files::List => {
                version.operator = Some(Operator::ListFiles);
                run(version)
            }
        }
    }
}
