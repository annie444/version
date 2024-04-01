use crate::cli::{
    files::FilesCommand,
    getset::{GetSet, GetSetBuild, GetSetRm},
};
use crate::{version::VersionFile, VersionError};
use clap::Subcommand;

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

impl VersionCommand {
    pub fn run(&self, version: &mut VersionFile) -> Result<(), VersionError> {
        match self {
            VersionCommand::Major(getset) => {
                version.key = Some(self.clone());
                getset.run(version)
            }
            VersionCommand::Minor(getset) => {
                version.key = Some(self.clone());
                getset.run(version)
            }
            VersionCommand::Patch(getset) => {
                version.key = Some(self.clone());
                getset.run(version)
            }
            VersionCommand::Alpha(getset) => {
                version.key = Some(self.clone());
                getset.run(version)
            }
            VersionCommand::Beta(getset) => {
                version.key = Some(self.clone());
                getset.run(version)
            }
            VersionCommand::RC(getset) => {
                version.key = Some(self.clone());
                getset.run(version)
            }
            VersionCommand::Build(getset) => {
                version.key = Some(self.clone());
                getset.run(version)
            }
            VersionCommand::Get => {
                version.key = Some(self.clone());
                version.run()
            }
            VersionCommand::File(file_cmd) => {
                version.key = Some(self.clone());
                file_cmd.run(version)
            }
        }
    }
}
