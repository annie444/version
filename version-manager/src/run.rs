use crate::{
    VersionError, VersionResult,
    files::{ModifyTrackedFiles, Package, VersionFile},
    version::{Operator, PrereleaseWrapper, Scope, SetTypes},
};
use semver::{BuildMetadata, Prerelease, Version};
use serde_json::json;
use std::path::PathBuf;

pub fn run(scope: Scope, file_path: PathBuf) -> VersionResult<()> {
    let file = VersionFile::load(file_path.clone())?;
    let mut ver = process_run(scope, file)?;
    ver.save(file_path)?;
    Ok(())
}

pub fn run_scopes<'a, T>(
    scope: Scope,
    version: &'a mut Version,
    files: &'a mut T,
) -> VersionResult<(&'a mut Version, &'a mut T, Option<(String, Scope)>)>
where
    T: ModifyTrackedFiles,
{
    match scope {
        Scope::Major(getset) => match getset {
            Operator::Set(set) => match set {
                SetTypes::Number(value) => version.major = value,
                SetTypes::AddNumber => version.major += 1,
                SetTypes::SubNumber => {
                    if version.major == 0 {
                        return Err(VersionError::NoNegatives);
                    }
                    version.major -= 1
                }
                _ => return Err(VersionError::InvalidOperation),
            },
            Operator::Get => println!("{}", version.major),
            Operator::Reset => {
                version.minor = 0;
                version.patch = 0;
                version.pre = Prerelease::EMPTY;
                version.build = BuildMetadata::EMPTY;
            }
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::Minor(getset) => match getset {
            Operator::Set(set) => match set {
                SetTypes::Number(value) => version.minor = value,
                SetTypes::AddNumber => version.minor += 1,
                SetTypes::SubNumber => {
                    if version.minor == 0 {
                        return Err(VersionError::NoNegatives);
                    }
                    version.minor -= 1
                }
                _ => return Err(VersionError::InvalidOperation),
            },
            Operator::Get => println!("{}", version.minor),
            Operator::Reset => {
                version.patch = 0;
                version.pre = Prerelease::EMPTY;
                version.build = BuildMetadata::EMPTY;
            }
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::Patch(getset) => match getset {
            Operator::Set(set) => match set {
                SetTypes::Number(value) => version.patch = value,
                SetTypes::AddNumber => version.patch += 1,
                SetTypes::SubNumber => {
                    if version.patch == 0 {
                        return Err(VersionError::NoNegatives);
                    }
                    version.patch -= 1
                }
                _ => return Err(VersionError::InvalidOperation),
            },
            Operator::Get => println!("{}", version.patch),
            Operator::Reset => {
                version.pre = Prerelease::EMPTY;
                version.build = BuildMetadata::EMPTY;
            }
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::Alpha(getsetrm) => match getsetrm {
            Operator::Set(set) => match set {
                SetTypes::Number(value) => {
                    version.pre = Prerelease::new(&format!("alpha.{}", value))?
                }
                SetTypes::AddNumber => {
                    match TryInto::<PrereleaseWrapper>::try_into(version.pre.clone()) {
                        Ok(mut wrapper) => {
                            wrapper.num += 1;
                            version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                        }
                        Err(_) => {
                            let wrapper = PrereleaseWrapper {
                                pre: "alpha".to_string(),
                                num: 0,
                            };
                            version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                        }
                    };
                }
                SetTypes::SubNumber => {
                    let mut wrapper = TryInto::<PrereleaseWrapper>::try_into(version.pre.clone())?;
                    wrapper.num -= 1;
                    if wrapper.num == 0 {
                        version.pre = Prerelease::EMPTY;
                    } else {
                        version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                    }
                }
                _ => return Err(VersionError::InvalidOperation),
            },
            Operator::Get => println!("{}", version.pre),
            Operator::Reset => {
                version.build = BuildMetadata::EMPTY;
            }
            Operator::Rm => version.pre = Prerelease::EMPTY,
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::Pre(getsetrm) => match getsetrm {
            Operator::Set(set) => match set {
                SetTypes::Number(value) => {
                    version.pre = Prerelease::new(&format!("pre.{}", value))?
                }
                SetTypes::AddNumber => {
                    match TryInto::<PrereleaseWrapper>::try_into(version.pre.clone()) {
                        Ok(mut wrapper) => {
                            wrapper.num += 1;
                            version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                        }
                        Err(_) => {
                            let wrapper = PrereleaseWrapper {
                                pre: "pre".to_string(),
                                num: 0,
                            };
                            version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                        }
                    };
                }
                SetTypes::SubNumber => {
                    let mut wrapper = TryInto::<PrereleaseWrapper>::try_into(version.pre.clone())?;
                    wrapper.num -= 1;
                    if wrapper.num == 0 {
                        version.pre = Prerelease::EMPTY;
                    } else {
                        version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                    }
                }
                _ => return Err(VersionError::InvalidOperation),
            },
            Operator::Get => println!("{}", version.pre),
            Operator::Reset => {
                version.build = BuildMetadata::EMPTY;
            }
            Operator::Rm => version.pre = Prerelease::EMPTY,
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::Beta(getsetrm) => match getsetrm {
            Operator::Set(set) => match set {
                SetTypes::Number(value) => {
                    version.pre = Prerelease::new(&format!("beta.{}", value))?
                }
                SetTypes::AddNumber => {
                    match TryInto::<PrereleaseWrapper>::try_into(version.pre.clone()) {
                        Ok(mut wrapper) => {
                            wrapper.num += 1;
                            version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                        }
                        Err(_) => {
                            let wrapper = PrereleaseWrapper {
                                pre: "beta".to_string(),
                                num: 0,
                            };
                            version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                        }
                    };
                }
                SetTypes::SubNumber => {
                    let mut wrapper = TryInto::<PrereleaseWrapper>::try_into(version.pre.clone())?;
                    wrapper.num -= 1;
                    if wrapper.num == 0 {
                        version.pre = Prerelease::EMPTY;
                    } else {
                        version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                    }
                }
                _ => return Err(VersionError::InvalidOperation),
            },
            Operator::Get => println!("{}", version.pre),
            Operator::Reset => {
                version.build = BuildMetadata::EMPTY;
            }
            Operator::Rm => version.pre = Prerelease::EMPTY,
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::RC(getsetrm) => match getsetrm {
            Operator::Set(set) => match set {
                SetTypes::Number(value) => version.pre = Prerelease::new(&format!("rc.{}", value))?,
                SetTypes::AddNumber => {
                    match TryInto::<PrereleaseWrapper>::try_into(version.pre.clone()) {
                        Ok(mut wrapper) => {
                            wrapper.num += 1;
                            version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                        }
                        Err(_) => {
                            let wrapper = PrereleaseWrapper {
                                pre: "rc".to_string(),
                                num: 0,
                            };
                            version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                        }
                    };
                }
                SetTypes::SubNumber => {
                    let mut wrapper = TryInto::<PrereleaseWrapper>::try_into(version.pre.clone())?;
                    wrapper.num -= 1;
                    if wrapper.num == 0 {
                        version.pre = Prerelease::EMPTY;
                    } else {
                        version.pre = TryInto::<Prerelease>::try_into(wrapper)?;
                    }
                }
                _ => return Err(VersionError::InvalidOperation),
            },
            Operator::Get => println!("{}", version.pre),
            Operator::Reset => {
                version.build = BuildMetadata::EMPTY;
            }
            Operator::Rm => {
                version.pre = Prerelease::EMPTY;
            }
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::Build(getsetbuild) => match getsetbuild {
            Operator::Set(set) => match set {
                SetTypes::String(value) => version.build = BuildMetadata::new(&value)?,
                _ => return Err(VersionError::InvalidOperation),
            },
            Operator::Get => println!("{}", version.build),
            Operator::Rm => version.build = BuildMetadata::EMPTY,
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::Get => println!("{}", version.to_string()),
        Scope::Version => println!("{}.{}.{}", version.major, version.minor, version.patch),
        Scope::Revision => println!("{}", version.pre),
        Scope::File(file_cmd) => match file_cmd {
            Operator::AddFile(set) => match set {
                SetTypes::NewFile(file) => files.add_tracked_file(file)?,
                _ => return Err(VersionError::InvalidOperation),
            },
            Operator::Update(file) => files.update_file(PathBuf::from(file))?,
            Operator::RmFile(file) => files.remove_tracked_file(PathBuf::from(file))?,
            Operator::UpdateAll => files.update_tracked_files()?,
            Operator::ListFiles => {
                let files = files.list_tracked_files()?;
                let file_str = json!(files).to_string();
                println!("{}", file_str);
            }
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::Set(set) => match set {
            Operator::SetVersion(version_str) => {
                *version = Version::parse(&version_str)?;
            }
            _ => return Err(VersionError::InvalidOperation),
        },
        Scope::Package(name, pkg) => return Ok((version, files, Some((name, *pkg)))),
        _ => return Err(VersionError::InvalidOperation),
    };
    files.sync_files()?;
    Ok((version, files, None))
}

pub fn process_run(scope: Scope, mut ver: VersionFile) -> VersionResult<VersionFile> {
    let mut ver_files = ver.clone();
    let mut ver_version = ver.version.clone();
    let (version, files, scope) = run_scopes(scope, &mut ver_version, &mut ver_files)?;
    ver = files.clone();
    ver.version = version.clone();
    if let Some((name, scope)) = scope {
        if scope == Scope::ListPackages {
            for (name, pkg) in ver.package.iter() {
                println!("{}: {}", name, pkg.version.to_string());
            }
            return Ok(ver);
        }
        if !ver.package.contains_key(&name) {
            ver.package.insert(name.clone(), Package::default());
        }
        let pkg = ver.get_package_mut(&name)?;
        let mut pkg_version = pkg.version.clone();
        let mut pkg_files = pkg.clone();
        let (version, files, _) = match scope {
            Scope::RmPackage => {
                ver.package.remove(&name);
                return Ok(ver);
            }
            _ => run_scopes(scope, &mut pkg_version, &mut pkg_files)?,
        };
        files.version = version.clone();
        ver.package.insert(name.clone(), files.clone());
    }
    Ok(ver)
}
