use crate::{cli::VersionCommand, files::TrackedFiles, VersionError, VersionResult};
use serde::{Deserialize, Serialize};
use std::{default::Default, env::current_dir, fs::File, io::prelude::*, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub alpha: Option<u8>,
    pub beta: Option<u8>,
    pub rc: Option<u8>,
    pub build: Option<String>,
    pub files: Option<Vec<TrackedFiles>>,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            major: 0,
            minor: 1,
            patch: 0,
            alpha: None,
            beta: None,
            rc: None,
            build: None,
            files: None,
        }
    }
}

impl Version {
    pub fn sync(&mut self) -> VersionResult<()> {
        self.sync_version_string()?;
        self.update_tracked_files()
    }

    pub fn get_version(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn get_revision(&self) -> String {
        let mut revision = "".to_string();
        if let Some(a) = self.alpha {
            if a > 0 {
                revision.push_str(&format!("alpha.{}", a));
            } else {
                revision.push_str("alpha");
            }
        }
        if let Some(b) = self.beta {
            if revision.len() > 0 {
                revision.push_str("-");
            }
            if b > 0 {
                revision.push_str(&format!("beta.{}", b));
            } else {
                revision.push_str("beta");
            }
        }
        if let Some(rc) = self.rc {
            if revision.len() > 0 {
                revision.push_str("-");
            }
            if rc > 0 {
                revision.push_str(&format!("rc.{}", rc));
            } else {
                revision.push_str("rc");
            }
        }
        revision
    }

    pub fn sync_version_string(&mut self) -> VersionResult<()> {
        self.version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(a) = self.alpha {
            let mut alpha: String = "-alpha".to_string();
            if a > 0 {
                alpha = format!("{}.{}", alpha, a);
            }
            self.version.push_str(&alpha);
        }
        if let Some(b) = self.beta {
            let mut beta = if self.version.contains('-') {
                ".beta".to_string()
            } else {
                "-beta".to_string()
            };
            if b > 0 {
                beta = format!("{}.{}", beta, b);
            }
            self.version.push_str(&beta);
        }
        if let Some(rc) = self.rc {
            let mut release_candidate = if self.version.contains('-') {
                ".rc".to_string()
            } else {
                "-rc".to_string()
            };
            if rc > 0 {
                release_candidate = format!("{}.{}", release_candidate, rc);
            }
            self.version.push_str(&release_candidate);
        }
        if let Some(build) = &self.build {
            self.version = format!("{}+{}", self.version, build);
        }
        println!("{}", self.version);
        Ok(())
    }

    pub fn update_tracked_files(&mut self) -> VersionResult<()> {
        if self.files.is_some() {
            for file in AsMut::<Vec<TrackedFiles>>::as_mut(self.files.as_mut().unwrap()).iter_mut()
            {
                file.update(self.version.clone())?;
            }
        }
        Ok(())
    }

    pub fn add_tracked_file(&mut self, file: TrackedFiles) -> VersionResult<()> {
        if self.files.is_some() {
            AsMut::<Vec<TrackedFiles>>::as_mut(self.files.as_mut().unwrap()).push(file);
        } else {
            self.files = Some(vec![file]);
        }
        Ok(())
    }

    pub fn remove_tracked_file(&mut self, file: PathBuf) -> VersionResult<()> {
        if self.files.is_some() {
            AsMut::<Vec<TrackedFiles>>::as_mut(self.files.as_mut().unwrap()).retain(|f| f != file);
        }
        Ok(())
    }

    pub fn update_file(&mut self, file: PathBuf) -> VersionResult<()> {
        if self.files.is_some() {
            for f in AsMut::<Vec<TrackedFiles>>::as_mut(self.files.as_mut().unwrap()).iter_mut() {
                if f == file {
                    f.update(self.version.clone())?;
                }
            }
        }
        Ok(())
    }

    pub fn set_major(&mut self, value: u8) -> VersionResult<()> {
        self.major = value;
        self.sync()
    }

    pub fn inc_major(&mut self) -> VersionResult<()> {
        self.major = self.major + 1;
        self.sync()
    }

    pub fn dec_major(&mut self) -> VersionResult<()> {
        self.major = self.major - 1;
        self.sync()
    }

    pub fn reset_major(&mut self) -> VersionResult<()> {
        self.minor = 0;
        self.patch = 0;
        self.alpha = None;
        self.beta = None;
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_minor(&mut self, value: u8) -> VersionResult<()> {
        self.minor = value;
        self.sync()
    }

    pub fn inc_minor(&mut self) -> VersionResult<()> {
        self.minor = self.minor + 1;
        self.sync()
    }

    pub fn dec_minor(&mut self) -> VersionResult<()> {
        self.minor = self.minor - 1;
        self.sync()
    }

    pub fn reset_minor(&mut self) -> VersionResult<()> {
        self.patch = 0;
        self.alpha = None;
        self.beta = None;
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_patch(&mut self, value: u8) -> VersionResult<()> {
        self.patch = value;
        self.sync()
    }

    pub fn inc_patch(&mut self) -> VersionResult<()> {
        self.patch = self.patch + 1;
        self.sync()
    }

    pub fn dec_patch(&mut self) -> VersionResult<()> {
        self.patch = self.patch - 1;
        self.sync()
    }

    pub fn reset_patch(&mut self) -> VersionResult<()> {
        self.alpha = None;
        self.beta = None;
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_alpha(&mut self, value: u8) -> VersionResult<()> {
        self.alpha = Some(value);
        self.sync()
    }

    pub fn inc_alpha(&mut self) -> VersionResult<()> {
        self.alpha = match self.alpha {
            Some(op) => Some(op + 1),
            None => Some(0),
        };
        self.sync()
    }

    pub fn dec_alpha(&mut self) -> VersionResult<()> {
        self.alpha = match self.alpha {
            Some(op) => {
                if op == 0 {
                    None
                } else {
                    Some(op - 1)
                }
            }
            None => Err(VersionError::NoNegatives)?,
        };
        self.sync()
    }

    pub fn rm_alpha(&mut self) -> VersionResult<()> {
        self.alpha = None;
        self.sync()
    }

    pub fn reset_alpha(&mut self) -> VersionResult<()> {
        self.beta = None;
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_beta(&mut self, value: u8) -> VersionResult<()> {
        self.beta = Some(value);
        self.sync()
    }

    pub fn inc_beta(&mut self) -> VersionResult<()> {
        self.beta = match self.beta {
            Some(op) => Some(op + 1),
            None => Some(0),
        };
        self.sync()
    }

    pub fn dec_beta(&mut self) -> VersionResult<()> {
        self.beta = match self.beta {
            Some(op) => {
                if op == 0 {
                    None
                } else {
                    Some(op - 1)
                }
            }
            None => Err(VersionError::NoNegatives)?,
        };
        self.sync()
    }

    pub fn rm_beta(&mut self) -> VersionResult<()> {
        self.beta = None;
        self.sync()
    }

    pub fn reset_beta(&mut self) -> VersionResult<()> {
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_rc(&mut self, value: u8) -> VersionResult<()> {
        self.rc = Some(value);
        self.sync()
    }

    pub fn inc_rc(&mut self) -> VersionResult<()> {
        self.rc = match self.rc {
            Some(op) => Some(op + 1),
            None => Some(0),
        };
        self.sync()
    }

    pub fn dec_rc(&mut self) -> VersionResult<()> {
        self.rc = match self.rc {
            Some(op) => {
                if op == 0 {
                    None
                } else {
                    Some(op - 1)
                }
            }
            None => Err(VersionError::NoNegatives)?,
        };
        self.sync()
    }

    pub fn rm_rc(&mut self) -> VersionResult<()> {
        self.rc = None;
        self.sync()
    }

    pub fn reset_rc(&mut self) -> VersionResult<()> {
        self.build = None;
        self.sync()
    }

    pub fn set_build(&mut self, value: String) -> VersionResult<()> {
        self.build = Some(value);
        self.sync()
    }

    pub fn rm_build(&mut self) -> VersionResult<()> {
        self.build = None;
        self.sync()
    }
}

pub enum Operator {
    Set(SetTypes),
    Rm,
    Reset,
    Get,
    AddFile,
    Update,
    RmFile,
    UpdateAll,
    ListFiles,
}

pub enum SetTypes {
    AddNumber,
    SubNumber,
    Number(u8),
    String(String),
    NewFile(TrackedFiles),
}

pub struct VersionFile {
    pub key: Option<VersionCommand>,
    pub operator: Option<Operator>,
    pub value: Option<SetTypes>,
    pub ver: Version,
}

impl VersionFile {
    pub fn load(version_file: PathBuf) -> VersionResult<Self> {
        let ver: Version = match File::open(version_file.clone()) {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(VersionError::IoError(e));
                    }
                }
                match toml::from_str(&contents) {
                    Ok(ver) => ver,
                    Err(e) => return Err(VersionError::TomlDeError(e)),
                }
            }
            Err(_) => match File::create(version_file) {
                Ok(_) => Version::default(),
                Err(e) => return Err(VersionError::IoError(e)),
            },
        };

        Ok(Self {
            ver,
            key: None,
            operator: None,
            value: None,
        })
    }

    pub fn save(&mut self) -> VersionResult<()> {
        self.save_version()?;
        self.save_files()?;
        Ok(())
    }

    fn save_version(&self) -> VersionResult<()> {
        let version = match toml::to_string_pretty(&self.ver) {
            Ok(v) => v,
            Err(e) => {
                return Err(VersionError::TomlSerError(e));
            }
        };
        let curr_dir = match current_dir() {
            Ok(curr_dir) => curr_dir,
            Err(e) => {
                return Err(VersionError::IoError(e));
            }
        };
        let version_file = curr_dir.join("VERSION.toml");
        let mut file = match File::options()
            .write(true)
            .truncate(true)
            .open(version_file)
        {
            Ok(file) => file,
            Err(e) => {
                return Err(VersionError::IoError(e));
            }
        };
        match file.write_all(version.as_bytes()) {
            Ok(_) => match file.flush() {
                Ok(_) => Ok(()),
                Err(e) => Err(VersionError::IoError(e)),
            },
            Err(e) => Err(VersionError::IoError(e)),
        }
    }

    fn save_files(&mut self) -> VersionResult<()> {
        if self.ver.files.is_some() {
            for file in
                AsMut::<Vec<TrackedFiles>>::as_mut(self.ver.files.as_mut().unwrap()).iter_mut()
            {
                file.update(self.ver.version.clone())?;
            }
        }
        Ok(())
    }
}

pub fn run(version: &mut VersionFile) -> VersionResult<()> {
    match &version.key {
        Some(key) => match key {
            VersionCommand::Major(_) => match &version.operator {
                Some(op) => match op {
                    Operator::Set(set) => match set {
                        SetTypes::Number(value) => version.ver.set_major(*value),
                        SetTypes::AddNumber => version.ver.inc_major(),
                        SetTypes::SubNumber => version.ver.dec_major(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    Operator::Get => {
                        println!("{}", version.ver.major);

                        Ok(())
                    }
                    Operator::Reset => version.ver.reset_major(),
                    _ => Err(VersionError::InvalidOperation),
                },
                None => Err(VersionError::IncompleteCommand),
            },
            VersionCommand::Minor(_) => match &version.operator {
                Some(op) => match op {
                    Operator::Set(set) => match set {
                        SetTypes::Number(value) => version.ver.set_minor(*value),
                        SetTypes::AddNumber => version.ver.inc_minor(),
                        SetTypes::SubNumber => version.ver.dec_minor(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    Operator::Get => {
                        println!("{}", version.ver.minor);

                        Ok(())
                    }
                    Operator::Reset => version.ver.reset_minor(),
                    _ => Err(VersionError::InvalidOperation),
                },
                None => Err(VersionError::IncompleteCommand),
            },
            VersionCommand::Patch(_) => match &version.operator {
                Some(op) => match op {
                    Operator::Set(set) => match set {
                        SetTypes::Number(value) => version.ver.set_patch(*value),
                        SetTypes::AddNumber => version.ver.inc_patch(),
                        SetTypes::SubNumber => version.ver.dec_patch(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    Operator::Get => {
                        println!("{}", version.ver.patch);

                        Ok(())
                    }
                    Operator::Reset => version.ver.reset_patch(),
                    _ => Err(VersionError::InvalidOperation),
                },
                None => Err(VersionError::IncompleteCommand),
            },
            VersionCommand::Alpha(_) => match &version.operator {
                Some(op) => match op {
                    Operator::Set(set) => match set {
                        SetTypes::Number(value) => version.ver.set_alpha(*value),
                        SetTypes::AddNumber => version.ver.inc_alpha(),
                        SetTypes::SubNumber => version.ver.dec_alpha(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    Operator::Get => {
                        if version.ver.alpha.is_some() {
                            println!("{}", version.ver.alpha.unwrap());
                        } else {
                        }

                        Ok(())
                    }
                    Operator::Reset => version.ver.reset_alpha(),
                    Operator::Rm => version.ver.rm_alpha(),
                    _ => Err(VersionError::InvalidOperation),
                },
                None => Err(VersionError::IncompleteCommand),
            },
            VersionCommand::Beta(_) => match &version.operator {
                Some(op) => match op {
                    Operator::Set(set) => match set {
                        SetTypes::Number(value) => version.ver.set_beta(*value),
                        SetTypes::AddNumber => version.ver.inc_beta(),
                        SetTypes::SubNumber => version.ver.dec_beta(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    Operator::Get => {
                        if version.ver.beta.is_some() {
                            println!("{}", version.ver.beta.unwrap());
                        } else {
                        }

                        Ok(())
                    }
                    Operator::Reset => version.ver.reset_beta(),
                    Operator::Rm => version.ver.rm_beta(),
                    _ => Err(VersionError::InvalidOperation),
                },
                None => Err(VersionError::IncompleteCommand),
            },
            VersionCommand::RC(_) => match &version.operator {
                Some(op) => match op {
                    Operator::Set(set) => match set {
                        SetTypes::Number(value) => version.ver.set_rc(*value),
                        SetTypes::AddNumber => version.ver.inc_rc(),
                        SetTypes::SubNumber => version.ver.dec_rc(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    Operator::Get => {
                        if version.ver.rc.is_some() {
                            println!("{}", version.ver.rc.unwrap());
                        } else {
                        }

                        Ok(())
                    }
                    Operator::Reset => version.ver.reset_rc(),
                    Operator::Rm => version.ver.rm_rc(),
                    _ => Err(VersionError::InvalidOperation),
                },
                None => Err(VersionError::IncompleteCommand),
            },
            VersionCommand::Build(_) => match &version.operator {
                Some(op) => match op {
                    Operator::Set(set) => match set {
                        SetTypes::String(value) => version.ver.set_build(value.to_string()),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    Operator::Get => {
                        println!("{}", version.ver.build.clone().unwrap_or("n/a".to_string()));

                        Ok(())
                    }
                    Operator::Rm => version.ver.rm_build(),
                    _ => Err(VersionError::InvalidOperation),
                },
                None => Err(VersionError::IncompleteCommand),
            },
            VersionCommand::Get => {
                println!("{}", version.ver.version);

                Ok(())
            }
            VersionCommand::Version => {
                println!("{}", version.ver.get_version());

                Ok(())
            }
            VersionCommand::Revision => {
                println!("{}", version.ver.get_revision());

                Ok(())
            }
            VersionCommand::File(_) => match &version.operator {
                Some(op) => match op {
                    Operator::AddFile => match &version.value {
                        Some(value) => match value {
                            SetTypes::NewFile(file) => version.ver.add_tracked_file(file.clone()),
                            _ => Err(VersionError::InvalidOperation),
                        },
                        None => Err(VersionError::IncompleteCommand),
                    },
                    Operator::RmFile => match &version.value {
                        Some(value) => match value {
                            SetTypes::String(value) => {
                                version.ver.remove_tracked_file(value.into())
                            }
                            _ => Err(VersionError::InvalidOperation),
                        },
                        None => Err(VersionError::IncompleteCommand),
                    },
                    Operator::Update => match &version.value {
                        Some(value) => match value {
                            SetTypes::String(file) => version.ver.update_file(file.into()),
                            _ => Err(VersionError::InvalidOperation),
                        },
                        None => Err(VersionError::IncompleteCommand),
                    },
                    Operator::UpdateAll => version.ver.update_tracked_files(),
                    Operator::ListFiles => {
                        if version.ver.files.is_some() {
                            for file in version.ver.files.as_ref().unwrap().iter() {
                                println!("{}", file.file);
                            }
                        }
                        Ok(())
                    }
                    _ => Err(VersionError::InvalidOperation),
                },
                None => Err(VersionError::IncompleteCommand),
            },
        },
        _ => Err(VersionError::InvalidOperation),
    }
}
