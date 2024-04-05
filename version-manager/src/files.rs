use crate::{VersionError, VersionResult};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{remove_file, rename, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Lines, Write},
    path::PathBuf,
};

#[derive(Deserialize, Serialize, Clone)]
pub struct TrackedFiles {
    pub file: String,
    pub expr: String,
}

impl TrackedFiles {
    pub fn new(file: String, expr: String) -> Self {
        TrackedFiles { file, expr }
    }

    pub fn new_from_path(file: PathBuf, expr: String) -> Self {
        TrackedFiles {
            file: file.to_string_lossy().to_string(),
            expr,
        }
    }

    pub fn new_from_re(file: String, expr: Regex) -> Self {
        TrackedFiles {
            file,
            expr: expr.as_str().to_string(),
        }
    }

    pub fn new_from_path_and_regex(file: PathBuf, expr: Regex) -> Self {
        TrackedFiles {
            file: file.to_string_lossy().to_string(),
            expr: expr.as_str().to_string(),
        }
    }

    pub fn open(&mut self) -> VersionResult<File> {
        let cd = match env::current_dir() {
            Ok(cd) => cd,
            Err(e) => return Err(VersionError::IoError(e)),
        };
        let path = cd.join(&self.file);
        match OpenOptions::new().read(true).open(&path) {
            Ok(file) => Ok(file),
            Err(e) => Err(VersionError::IoError(e)),
        }
    }

    pub fn open_tmp(&mut self) -> VersionResult<File> {
        let cd = match env::current_dir() {
            Ok(cd) => cd,
            Err(e) => return Err(VersionError::IoError(e)),
        };
        let mut path = cd.join(&self.file);
        path = path.with_extension("tmp");
        match OpenOptions::new().create(true).write(true).open(&path) {
            Ok(file) => Ok(file),
            Err(e) => Err(VersionError::IoError(e)),
        }
    }

    pub fn close(&mut self) -> VersionResult<()> {
        let cd = match env::current_dir() {
            Ok(cd) => cd,
            Err(e) => return Err(VersionError::IoError(e)),
        };
        let old_path = cd.join(&self.file);
        let new_path = old_path.with_extension("tmp");
        match remove_file(&old_path) {
            Ok(_) => (),
            Err(e) => return Err(VersionError::IoError(e)),
        };
        match rename(new_path, old_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(VersionError::IoError(e)),
        }
    }

    pub fn read_lines(&mut self) -> VersionResult<Lines<BufReader<File>>> {
        let file = self.open()?;
        let reader = BufReader::new(file);
        Ok(reader.lines())
    }

    pub fn writer(&mut self) -> VersionResult<BufWriter<File>> {
        let file = self.open_tmp()?;
        Ok(BufWriter::new(file))
    }

    pub fn update(&mut self, version: String) -> VersionResult<()> {
        let mut writer = self.writer()?;
        let regex = match Regex::new(&self.expr) {
            Ok(re) => re,
            Err(e) => return Err(VersionError::RegexError(e)),
        };
        let mut updated = false;
        for line in match self.read_lines() {
            Ok(lines) => lines,
            Err(e) => return Err(e),
        } {
            let line = match line {
                Ok(line) => format!("{}\n", line),
                Err(e) => return Err(VersionError::IoError(e)),
            };
            if !updated {
                if let Some(matches) = regex.captures(&line) {
                    let new = line.replace(&matches[1], version.as_str());
                    match writer.write(new.as_bytes()) {
                        Ok(_) => (),
                        Err(e) => return Err(VersionError::IoError(e)),
                    };
                    updated = true;
                } else {
                    match writer.write(line.as_bytes()) {
                        Ok(_) => (),
                        Err(e) => return Err(VersionError::IoError(e)),
                    };
                }
            } else {
                match writer.write(line.as_bytes()) {
                    Ok(_) => (),
                    Err(e) => return Err(VersionError::IoError(e)),
                };
            }
        }
        match writer.flush() {
            Ok(_) => (),
            Err(e) => return Err(VersionError::IoError(e)),
        };
        drop(writer);
        self.close()
    }
}

impl PartialEq<String> for TrackedFiles {
    fn eq(&self, other: &String) -> bool {
        self.file == *other
    }
}

impl PartialEq<PathBuf> for TrackedFiles {
    fn eq(&self, other: &PathBuf) -> bool {
        let path = PathBuf::from(self.file.clone());
        path == *other
    }
}

impl PartialEq<String> for &TrackedFiles {
    fn eq(&self, other: &String) -> bool {
        self.file == *other
    }
}

impl PartialEq<PathBuf> for &TrackedFiles {
    fn eq(&self, other: &PathBuf) -> bool {
        let path = PathBuf::from(self.file.clone());
        path == *other
    }
}

impl PartialEq<String> for &mut TrackedFiles {
    fn eq(&self, other: &String) -> bool {
        self.file == *other
    }
}

impl PartialEq<PathBuf> for &mut TrackedFiles {
    fn eq(&self, other: &PathBuf) -> bool {
        let path = PathBuf::from(self.file.clone());
        path == *other
    }
}
