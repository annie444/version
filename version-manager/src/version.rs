use crate::VersionError;
use crate::files::TrackedFiles;
use semver::Prerelease;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Set(SetTypes),
    Rm,
    Reset,
    Get,
    AddFile(SetTypes),
    SetVersion(String),
    Update(String),
    RmFile(String),
    UpdateAll,
    ListFiles,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetTypes {
    AddNumber,
    SubNumber,
    Number(u64),
    String(String),
    NewFile(TrackedFiles),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Major(Operator),
    Minor(Operator),
    Patch(Operator),
    Alpha(Operator),
    Beta(Operator),
    Pre(Operator),
    RC(Operator),
    Build(Operator),
    Get,
    Set(Operator),
    Version,
    Revision,
    File(Operator),
    Package(String, Box<Scope>),
    ListPackages,
    RmPackage,
}

pub struct PrereleaseWrapper {
    pub pre: String,
    pub num: u64,
}

impl PrereleaseWrapper {
    pub fn new(pre: String, num: u64) -> Self {
        Self { pre, num }
    }
}

impl TryFrom<PrereleaseWrapper> for Prerelease {
    type Error = semver::Error;

    fn try_from(wrapper: PrereleaseWrapper) -> Result<Self, Self::Error> {
        Prerelease::new(&format!("{}.{}", wrapper.pre, wrapper.num))
    }
}

impl TryFrom<Prerelease> for PrereleaseWrapper {
    type Error = VersionError;

    fn try_from(prerelease: Prerelease) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = prerelease.as_str().split('.').collect();
        if parts.len() != 2 {
            Err(VersionError::InvalidPrerelease(prerelease.to_string()))
        } else {
            let pre = parts[0].to_string();
            let num = parts[1]
                .parse::<u64>()
                .map_err(|_| VersionError::InvalidPrerelease(prerelease.to_string()))?;
            Ok(PrereleaseWrapper::new(pre, num))
        }
    }
}

impl ToString for PrereleaseWrapper {
    fn to_string(&self) -> String {
        format!("{}.{}", self.pre, self.num)
    }
}
