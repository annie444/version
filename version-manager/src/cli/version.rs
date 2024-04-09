use crate::{
    cli::{
        files::FilesCommand,
        getset::{GetSet, GetSetBuild, GetSetRm},
    },
    version::{run, VersionFile},
    CommandRun, VersionResult,
};
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
    /// Get just the version number as a string with no revision or build identifiers
    Version,
    /// Get just the revision number as a string with no build identifiers
    Revision,
    /// Track and update the version number in a file
    File(FilesCommand),
}

impl CommandRun for VersionCommand {
    fn run(&self, version: &mut VersionFile) -> VersionResult<()> {
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
                run(version)
            }
            VersionCommand::File(file_cmd) => {
                version.key = Some(self.clone());
                file_cmd.run(version)
            }
            VersionCommand::Version => {
                version.key = Some(self.clone());
                run(version)
            }
            VersionCommand::Revision => {
                version.key = Some(self.clone());
                run(version)
            }
        }
    }
}
