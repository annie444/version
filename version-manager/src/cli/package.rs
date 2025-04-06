use super::{
    files::FilesCommand,
    getset::{GetSet, GetSetBuild, GetSetRm, SetVer},
};
use crate::{VersionError, version::Scope};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone, PartialEq)]
#[command(arg_required_else_help(true))]
pub struct PackageCommand {
    /// The package name to track
    pub package_name: Option<String>,
    #[clap(subcommand)]
    pub command: PackageOperation,
}

impl TryFrom<PackageCommand> for Scope {
    type Error = VersionError;

    fn try_from(cmd: PackageCommand) -> Result<Self, Self::Error> {
        Ok(Scope::Package(
            (&cmd).try_into()?,
            Box::new(cmd.command.try_into()?),
        ))
    }
}

impl TryFrom<&PackageCommand> for Scope {
    type Error = VersionError;

    fn try_from(cmd: &PackageCommand) -> Result<Self, Self::Error> {
        Ok(Scope::Package(
            cmd.try_into()?,
            Box::new((&cmd.command).try_into()?),
        ))
    }
}

impl TryFrom<PackageCommand> for String {
    type Error = VersionError;

    fn try_from(cmd: PackageCommand) -> Result<Self, Self::Error> {
        match cmd.package_name {
            Some(ref name) => Ok(name.clone()),
            None => {
                if cmd.command == PackageOperation::List {
                    Ok("".to_string())
                } else {
                    return Err(VersionError::PackageNameRequired);
                }
            }
        }
    }
}

impl TryFrom<&PackageCommand> for String {
    type Error = VersionError;

    fn try_from(cmd: &PackageCommand) -> Result<Self, Self::Error> {
        match cmd.package_name {
            Some(ref name) => Ok(name.clone()),
            None => {
                if cmd.command == PackageOperation::List {
                    Ok("".to_string())
                } else {
                    return Err(VersionError::PackageNameRequired);
                }
            }
        }
    }
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
#[command(arg_required_else_help(true))]
pub enum PackageOperation {
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
    /// Remove a package
    Rm,
    /// List tracked packages
    List,
}

impl TryFrom<PackageOperation> for Scope {
    type Error = VersionError;

    fn try_from(cmd: PackageOperation) -> Result<Self, Self::Error> {
        let scope = match cmd {
            PackageOperation::Major(getset) => Scope::Major(getset.try_into()?),
            PackageOperation::Minor(getset) => Scope::Minor(getset.try_into()?),
            PackageOperation::Patch(getset) => Scope::Patch(getset.try_into()?),
            PackageOperation::Alpha(getset) => Scope::Alpha(getset.try_into()?),
            PackageOperation::Beta(getset) => Scope::Beta(getset.try_into()?),
            PackageOperation::RC(getset) => Scope::RC(getset.try_into()?),
            PackageOperation::Build(getset) => Scope::Build(getset.try_into()?),
            PackageOperation::Get => Scope::Get,
            PackageOperation::Version => Scope::Version,
            PackageOperation::Revision => Scope::Revision,
            PackageOperation::File(file_cmd) => Scope::File(file_cmd.try_into()?),
            PackageOperation::Rm => Scope::RmPackage,
            PackageOperation::List => Scope::ListPackages,
            PackageOperation::Set(setver) => Scope::Set(setver.try_into()?),
        };
        Ok(scope)
    }
}

impl TryFrom<&PackageOperation> for Scope {
    type Error = VersionError;

    fn try_from(cmd: &PackageOperation) -> Result<Self, Self::Error> {
        let scope = match cmd {
            PackageOperation::Major(getset) => Scope::Major(getset.try_into()?),
            PackageOperation::Minor(getset) => Scope::Minor(getset.try_into()?),
            PackageOperation::Patch(getset) => Scope::Patch(getset.try_into()?),
            PackageOperation::Alpha(getset) => Scope::Alpha(getset.try_into()?),
            PackageOperation::Beta(getset) => Scope::Beta(getset.try_into()?),
            PackageOperation::RC(getset) => Scope::RC(getset.try_into()?),
            PackageOperation::Build(getset) => Scope::Build(getset.try_into()?),
            PackageOperation::Get => Scope::Get,
            PackageOperation::Version => Scope::Version,
            PackageOperation::Revision => Scope::Revision,
            PackageOperation::File(file_cmd) => Scope::File(file_cmd.try_into()?),
            PackageOperation::Rm => Scope::RmPackage,
            PackageOperation::List => Scope::ListPackages,
            PackageOperation::Set(setver) => Scope::Set(setver.try_into()?),
        };
        Ok(scope)
    }
}
