use crate::{VersionError, VersionResult};
use regex::Regex;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    fs::{File, OpenOptions, remove_file, rename},
    io::{BufRead, BufReader, BufWriter, Lines, Read, Write},
    path::PathBuf,
};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]
pub struct VersionFile {
    pub version: Version,
    pub files: Vec<TrackedFiles>,
    pub package: BTreeMap<String, Package>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]
pub struct Package {
    pub version: Version,
    pub files: Vec<TrackedFiles>,
}

impl Default for VersionFile {
    fn default() -> Self {
        VersionFile {
            version: Version::new(0, 1, 0),
            files: vec![],
            package: BTreeMap::new(),
        }
    }
}

impl Default for Package {
    fn default() -> Self {
        Package {
            version: Version::new(0, 1, 0),
            files: vec![],
        }
    }
}

pub trait ModifyTrackedFiles {
    fn sync_files(&self) -> VersionResult<()> {
        self.update_tracked_files()
    }
    fn update_tracked_files(&self) -> VersionResult<()>;
    fn add_tracked_file(&mut self, file: TrackedFiles) -> VersionResult<()>;
    fn remove_tracked_file(&mut self, file: PathBuf) -> VersionResult<()>;
    fn update_file(&self, file: PathBuf) -> VersionResult<()>;
    fn list_tracked_files(&self) -> VersionResult<Vec<TrackedFiles>>;
}

impl ModifyTrackedFiles for VersionFile {
    fn update_tracked_files(&self) -> VersionResult<()> {
        for file in self.files.iter() {
            file.update(self.version.to_string())?;
        }
        Ok(())
    }

    fn add_tracked_file(&mut self, file: TrackedFiles) -> VersionResult<()> {
        self.files.push(file);
        Ok(())
    }

    fn remove_tracked_file(&mut self, file: PathBuf) -> VersionResult<()> {
        self.files.retain(|f| f != file);
        Ok(())
    }

    fn update_file(&self, file: PathBuf) -> VersionResult<()> {
        for f in self.files.iter() {
            if f == file {
                f.update(self.version.to_string())?;
            }
        }
        Ok(())
    }

    fn list_tracked_files(&self) -> VersionResult<Vec<TrackedFiles>> {
        return Ok(self.files.clone());
    }
}

impl ModifyTrackedFiles for Package {
    fn update_tracked_files(&self) -> VersionResult<()> {
        for file in self.files.iter() {
            file.update(self.version.to_string())?;
        }
        Ok(())
    }

    fn add_tracked_file(&mut self, file: TrackedFiles) -> VersionResult<()> {
        self.files.push(file);
        Ok(())
    }

    fn remove_tracked_file(&mut self, file: PathBuf) -> VersionResult<()> {
        self.files.retain(|f| f != file);
        Ok(())
    }

    fn update_file(&self, file: PathBuf) -> VersionResult<()> {
        for f in self.files.iter() {
            if f == file {
                f.update(self.version.to_string())?;
            }
        }
        Ok(())
    }

    fn list_tracked_files(&self) -> VersionResult<Vec<TrackedFiles>> {
        return Ok(self.files.clone());
    }
}

impl VersionFile {
    pub fn get_package(&self, name: &str) -> VersionResult<&Package> {
        if let Some(ref pkg) = self.package.get(name) {
            return Ok(pkg);
        }
        Err(VersionError::InvalidCommand)
    }

    pub fn get_package_mut(&mut self, name: &str) -> VersionResult<&mut Package> {
        if let Some(pkg) = self.package.get_mut(name) {
            return Ok(pkg);
        }
        Err(VersionError::InvalidCommand)
    }

    pub fn load(version_file: PathBuf) -> VersionResult<Self> {
        let ver: Self = match File::open(version_file.clone()) {
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
                Ok(_) => Self::default(),
                Err(e) => return Err(VersionError::IoError(e)),
            },
        };

        Ok(ver)
    }

    pub fn save(&mut self, version_file: PathBuf) -> VersionResult<()> {
        self.save_version(version_file)?;
        self.save_files()?;
        Ok(())
    }

    fn save_version(&self, version_file: PathBuf) -> VersionResult<()> {
        let version = match toml::to_string_pretty(&self) {
            Ok(v) => v,
            Err(e) => {
                return Err(VersionError::TomlSerError(e));
            }
        };
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

    fn save_files(&self) -> VersionResult<()> {
        self.sync_files()?;
        for (_, pkg) in self.package.iter() {
            pkg.sync_files()?;
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, PartialOrd)]
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

    pub fn open(&self) -> VersionResult<File> {
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

    pub fn open_tmp(&self) -> VersionResult<File> {
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

    pub fn close(&self) -> VersionResult<()> {
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

    pub fn read_lines(&self) -> VersionResult<Lines<BufReader<File>>> {
        let file = self.open()?;
        let reader = BufReader::new(file);
        Ok(reader.lines())
    }

    pub fn writer(&self) -> VersionResult<BufWriter<File>> {
        let file = self.open_tmp()?;
        Ok(BufWriter::new(file))
    }

    pub fn update(&self, version: String) -> VersionResult<()> {
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
        let path = PathBuf::from(&self.file);
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
        let path = PathBuf::from(&self.file);
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
        let path = PathBuf::from(&self.file);
        path == *other
    }
}
