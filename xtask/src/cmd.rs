use clap::{
    builder::{NonEmptyStringValueParser, RangedU64ValueParser},
    value_parser, Args, Parser, Subcommand,
};
use clap_complete::Shell;
use clio::ClioPath;
use regex::Regex;

#[derive(Parser, Debug, Clone)]
#[command(arg_required_else_help(true), name = "version")]
/// A tool for managing the version of a project
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<VersionCommand>,
    #[arg(value_parser = value_parser!(Shell), exclusive = true)]
    /// Generate shell completions
    pub generator: Option<Shell>,
}

#[derive(Subcommand, Debug, Clone)]
/// Specify the version number to change
#[command(rename_all = "lower", arg_required_else_help(true))]
pub enum VersionCommand {
    /// Change the major version number
    Major(GetSet),
    /// Change the minor version number
    Minor(GetSet),
    /// Change the patch version number
    Patch(GetSet),
    /// Change the alpha identifier
    Alpha(GetSetRm),
    /// Change the beta identifier
    Beta(GetSetRm),
    /// Change the release candidate identifier
    RC(GetSetRm),
    /// Change the build identifier
    Build(GetSetBuild),
    /// Get the current version number as a full SemVer string
    Get,
    /// Track and update the version number in a file
    File(FilesCommand),
}

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

#[derive(Parser, Debug, Clone)]
/// Get or set the version number
#[command(arg_required_else_help(true))]
pub struct GetSet {
    #[command(subcommand)]
    pub command: GetSetCommand,
}

#[derive(Subcommand, Debug, Clone)]
/// Get or set the version number
pub enum GetSetCommand {
    /// Print the current version
    Get,
    /// Set the version number
    Set(Set),
    /// Reset the subversions
    Reset,
}

#[derive(Parser, Debug, Clone)]
/// Get or set the version number
#[command(arg_required_else_help(true))]
pub struct GetSetRm {
    #[command(subcommand)]
    pub command: GetSetRmCommand,
}

#[derive(Subcommand, Debug, Clone)]
/// Get or set the version number
pub enum GetSetRmCommand {
    /// Print the current version
    Get,
    /// Set the version number
    Set(Set),
    /// Remove the version identifier
    Rm,
}

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

#[derive(Parser, Debug, Clone)]
/// Get or set the build version
#[command(arg_required_else_help(true))]
pub struct GetSetBuild {
    #[command(subcommand)]
    pub command: GetSetBuildCommand,
}

#[derive(Subcommand, Debug, Clone)]
/// Get or set the build version
pub enum GetSetBuildCommand {
    Get,
    Set(SetBuild),
    Rm,
}

#[derive(Args, Debug, Clone)]
/// Set the build version
pub struct SetBuild {
    #[arg(value_parser = NonEmptyStringValueParser::new())]
    pub value: String,
}
