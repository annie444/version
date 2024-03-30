use crate::{cli::VersionCommand, VersionError};
use serde::{Deserialize, Serialize};
use std::{default::Default, env::current_dir, fs::File, io::prelude::*};

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
        }
    }
}

impl Version {
    pub fn sync(&mut self) -> Result<(), VersionError> {
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

    pub fn set_major(&mut self, value: u8) -> Result<(), VersionError> {
        self.major = value;
        self.sync()
    }

    pub fn inc_major(&mut self) -> Result<(), VersionError> {
        self.major = self.major + 1;
        self.sync()
    }

    pub fn dec_major(&mut self) -> Result<(), VersionError> {
        self.major = self.major - 1;
        self.sync()
    }

    pub fn reset_major(&mut self) -> Result<(), VersionError> {
        self.minor = 0;
        self.patch = 0;
        self.alpha = None;
        self.beta = None;
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_minor(&mut self, value: u8) -> Result<(), VersionError> {
        self.minor = value;
        self.sync()
    }

    pub fn inc_minor(&mut self) -> Result<(), VersionError> {
        self.minor = self.minor + 1;
        self.sync()
    }

    pub fn dec_minor(&mut self) -> Result<(), VersionError> {
        self.minor = self.minor - 1;
        self.sync()
    }

    pub fn reset_minor(&mut self) -> Result<(), VersionError> {
        self.patch = 0;
        self.alpha = None;
        self.beta = None;
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_patch(&mut self, value: u8) -> Result<(), VersionError> {
        self.patch = value;
        self.sync()
    }

    pub fn inc_patch(&mut self) -> Result<(), VersionError> {
        self.patch = self.patch + 1;
        self.sync()
    }

    pub fn dec_patch(&mut self) -> Result<(), VersionError> {
        self.patch = self.patch - 1;
        self.sync()
    }

    pub fn reset_patch(&mut self) -> Result<(), VersionError> {
        self.alpha = None;
        self.beta = None;
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_alpha(&mut self, value: u8) -> Result<(), VersionError> {
        self.alpha = Some(value);
        self.sync()
    }

    pub fn inc_alpha(&mut self) -> Result<(), VersionError> {
        self.alpha = match self.alpha {
            Some(op) => Some(op + 1),
            None => Some(0),
        };
        self.sync()
    }

    pub fn dec_alpha(&mut self) -> Result<(), VersionError> {
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

    pub fn rm_alpha(&mut self) -> Result<(), VersionError> {
        self.alpha = None;
        self.sync()
    }

    pub fn reset_alpha(&mut self) -> Result<(), VersionError> {
        self.beta = None;
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_beta(&mut self, value: u8) -> Result<(), VersionError> {
        self.beta = Some(value);
        self.sync()
    }

    pub fn inc_beta(&mut self) -> Result<(), VersionError> {
        self.beta = match self.beta {
            Some(op) => Some(op + 1),
            None => Some(0),
        };
        self.sync()
    }

    pub fn dec_beta(&mut self) -> Result<(), VersionError> {
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

    pub fn rm_beta(&mut self) -> Result<(), VersionError> {
        self.beta = None;
        self.sync()
    }

    pub fn reset_beta(&mut self) -> Result<(), VersionError> {
        self.rc = None;
        self.build = None;
        self.sync()
    }

    pub fn set_rc(&mut self, value: u8) -> Result<(), VersionError> {
        self.rc = Some(value);
        self.sync()
    }

    pub fn inc_rc(&mut self) -> Result<(), VersionError> {
        self.rc = match self.rc {
            Some(op) => Some(op + 1),
            None => Some(0),
        };
        self.sync()
    }

    pub fn dec_rc(&mut self) -> Result<(), VersionError> {
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

    pub fn rm_rc(&mut self) -> Result<(), VersionError> {
        self.rc = None;
        self.sync()
    }

    pub fn reset_rc(&mut self) -> Result<(), VersionError> {
        self.build = None;
        self.sync()
    }

    pub fn set_build(&mut self, value: String) -> Result<(), VersionError> {
        self.build = Some(value);
        self.sync()
    }

    pub fn rm_build(&mut self) -> Result<(), VersionError> {
        self.build = None;
        self.sync()
    }
}

pub enum Operator {
    Set(SetTypes),
    Rm,
    Reset,
    Get,
}

pub enum SetTypes {
    AddNumber,
    SubNumber,
    Number(u8),
    String(String),
}

pub struct VersionFile {
    pub key: Option<VersionCommand>,
    pub operator: Option<Operator>,
    pub value: Option<SetTypes>,
    pub ver: Version,
}

impl VersionFile {
    pub fn load() -> Result<Self, VersionError> {
        let curr_dir = match current_dir() {
            Ok(curr_dir) => curr_dir,
            Err(e) => {
                return Err(VersionError::IoError(e));
            }
        };
        let version_file = curr_dir.join("VERSION.toml");
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

    pub fn save(&mut self) -> Result<(), VersionError> {
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

    pub fn run(&mut self) -> Result<(), VersionError> {
        match &self.key {
            Some(key) => match key {
                VersionCommand::Major(_) => match &self.operator {
                    Some(op) => match op {
                        Operator::Set(set) => match set {
                            SetTypes::Number(value) => self.ver.set_major(*value),
                            SetTypes::AddNumber => self.ver.inc_major(),
                            SetTypes::SubNumber => self.ver.dec_major(),
                            _ => Err(VersionError::InvalidOperation),
                        },
                        Operator::Get => {
                            println!("{}", self.ver.major);
                            Ok(())
                        }
                        Operator::Reset => self.ver.reset_major(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    None => Err(VersionError::IncompleteCommand),
                },
                VersionCommand::Minor(_) => match &self.operator {
                    Some(op) => match op {
                        Operator::Set(set) => match set {
                            SetTypes::Number(value) => self.ver.set_minor(*value),
                            SetTypes::AddNumber => self.ver.inc_minor(),
                            SetTypes::SubNumber => self.ver.dec_minor(),
                            _ => Err(VersionError::InvalidOperation),
                        },
                        Operator::Get => {
                            println!("{}", self.ver.major);
                            Ok(())
                        }
                        Operator::Reset => self.ver.reset_minor(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    None => Err(VersionError::IncompleteCommand),
                },
                VersionCommand::Patch(_) => match &self.operator {
                    Some(op) => match op {
                        Operator::Set(set) => match set {
                            SetTypes::Number(value) => self.ver.set_patch(*value),
                            SetTypes::AddNumber => self.ver.inc_patch(),
                            SetTypes::SubNumber => self.ver.dec_patch(),
                            _ => Err(VersionError::InvalidOperation),
                        },
                        Operator::Get => {
                            println!("{}", self.ver.major);
                            Ok(())
                        }
                        Operator::Reset => self.ver.reset_patch(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    None => Err(VersionError::IncompleteCommand),
                },
                VersionCommand::Alpha(_) => match &self.operator {
                    Some(op) => match op {
                        Operator::Set(set) => match set {
                            SetTypes::Number(value) => self.ver.set_alpha(*value),
                            SetTypes::AddNumber => self.ver.inc_alpha(),
                            SetTypes::SubNumber => self.ver.dec_alpha(),
                            _ => Err(VersionError::InvalidOperation),
                        },
                        Operator::Get => {
                            println!("{}", self.ver.major);
                            Ok(())
                        }
                        Operator::Reset => self.ver.reset_alpha(),
                        Operator::Rm => self.ver.rm_alpha(),
                    },
                    None => Err(VersionError::IncompleteCommand),
                },
                VersionCommand::Beta(_) => match &self.operator {
                    Some(op) => match op {
                        Operator::Set(set) => match set {
                            SetTypes::Number(value) => self.ver.set_beta(*value),
                            SetTypes::AddNumber => self.ver.inc_beta(),
                            SetTypes::SubNumber => self.ver.dec_beta(),
                            _ => Err(VersionError::InvalidOperation),
                        },
                        Operator::Get => {
                            println!("{}", self.ver.major);
                            Ok(())
                        }
                        Operator::Reset => self.ver.reset_beta(),
                        Operator::Rm => self.ver.rm_beta(),
                    },
                    None => Err(VersionError::IncompleteCommand),
                },
                VersionCommand::RC(_) => match &self.operator {
                    Some(op) => match op {
                        Operator::Set(set) => match set {
                            SetTypes::Number(value) => self.ver.set_rc(*value),
                            SetTypes::AddNumber => self.ver.inc_rc(),
                            SetTypes::SubNumber => self.ver.dec_rc(),
                            _ => Err(VersionError::InvalidOperation),
                        },
                        Operator::Get => {
                            println!("{}", self.ver.major);
                            Ok(())
                        }
                        Operator::Reset => self.ver.reset_rc(),
                        Operator::Rm => self.ver.rm_rc(),
                    },
                    None => Err(VersionError::IncompleteCommand),
                },
                VersionCommand::Build(_) => match &self.operator {
                    Some(op) => match op {
                        Operator::Set(set) => match set {
                            SetTypes::String(value) => self.ver.set_build(value.to_string()),
                            _ => Err(VersionError::InvalidOperation),
                        },
                        Operator::Get => {
                            println!("{}", self.ver.major);
                            Ok(())
                        }
                        Operator::Rm => self.ver.rm_build(),
                        _ => Err(VersionError::InvalidOperation),
                    },
                    None => Err(VersionError::IncompleteCommand),
                },
                VersionCommand::Get => {
                    println!("{}", self.ver.version);
                    Ok(())
                }
            },
            _ => Err(VersionError::InvalidOperation),
        }
    }
}
