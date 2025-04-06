use crate::{
    VersionError,
    cli::{
        files::FilesCommand,
        getset::{GetSet, GetSetBuild, GetSetRm, SetVer},
        package::PackageCommand,
    },
    version::Scope,
};
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone, PartialEq)]
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
    /// Set the version number to a specific version
    Set(SetVer),
    /// Get just the version number as a string with no revision or build identifiers
    Version,
    /// Get just the revision number as a string with no build identifiers
    Revision,
    /// Track and update the version number in a file
    File(FilesCommand),
    /// Track and update the version number in a file
    Package(PackageCommand),
}

impl TryFrom<&VersionCommand> for Scope {
    type Error = VersionError;

    fn try_from(cmd: &VersionCommand) -> Result<Self, VersionError> {
        let scope = match cmd {
            VersionCommand::Major(getset) => Scope::Major(getset.try_into()?),
            VersionCommand::Minor(getset) => Scope::Minor(getset.try_into()?),
            VersionCommand::Patch(getset) => Scope::Patch(getset.try_into()?),
            VersionCommand::Alpha(getset) => Scope::Alpha(getset.try_into()?),
            VersionCommand::Beta(getset) => Scope::Beta(getset.try_into()?),
            VersionCommand::RC(getset) => Scope::RC(getset.try_into()?),
            VersionCommand::Build(getset) => Scope::Build(getset.try_into()?),
            VersionCommand::Get => Scope::Get,
            VersionCommand::Version => Scope::Version,
            VersionCommand::Revision => Scope::Revision,
            VersionCommand::File(file_cmd) => Scope::File(file_cmd.try_into()?),
            VersionCommand::Package(package_cmd) => package_cmd.try_into()?,
            VersionCommand::Set(setver) => Scope::Set(setver.try_into()?),
        };
        Ok(scope)
    }
}

impl TryFrom<VersionCommand> for Scope {
    type Error = VersionError;

    fn try_from(cmd: VersionCommand) -> Result<Self, VersionError> {
        let scope = match cmd {
            VersionCommand::Major(getset) => Scope::Major(getset.try_into()?),
            VersionCommand::Minor(getset) => Scope::Minor(getset.try_into()?),
            VersionCommand::Patch(getset) => Scope::Patch(getset.try_into()?),
            VersionCommand::Alpha(getset) => Scope::Alpha(getset.try_into()?),
            VersionCommand::Beta(getset) => Scope::Beta(getset.try_into()?),
            VersionCommand::RC(getset) => Scope::RC(getset.try_into()?),
            VersionCommand::Build(getset) => Scope::Build(getset.try_into()?),
            VersionCommand::Get => Scope::Get,
            VersionCommand::Version => Scope::Version,
            VersionCommand::Revision => Scope::Revision,
            VersionCommand::File(file_cmd) => Scope::File(file_cmd.try_into()?),
            VersionCommand::Package(package_cmd) => package_cmd.try_into()?,
            VersionCommand::Set(setver) => Scope::Set(setver.try_into()?),
        };
        Ok(scope)
    }
}
