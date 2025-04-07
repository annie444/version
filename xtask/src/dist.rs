use crate::targets::Targets;
use crate::{CRATE_NAME, PACKAGE_NAME, error::Result, project_root};
use clap::CommandFactory;
use os_info::Type;
use std::{env, fs, io::prelude::*, path::PathBuf, process::Command, process::Stdio};
use version_manager::cli::Cli;

pub fn dist_dir(target: Option<&str>) -> PathBuf {
    match target {
        Some(target) => project_root().join(format!("target/dist/{}", target)),
        None => project_root().join("target/dist"),
    }
}
pub fn dist() -> Result<()> {
    if dist_dir(None).exists() {
        let _ = fs::remove_dir_all(&dist_dir(None));
    }
    fs::create_dir_all(&dist_dir(None))?;

    dist_binary(None, None)?;
    dist_manpage(None)?;
    dist_readme(None)?;
    dist_license(None)?;
    dist_changelog(None)?;

    Ok(())
}

pub fn dist_binary_cargo(target: Targets) -> Result<()> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&[
            "build",
            "--package",
            CRATE_NAME,
            "--target",
            target.into(),
            "--release",
        ])
        .status()?;

    if !status.success() {
        return Err(format!("Cargo build failed with exit code: {:?}", status.code()).into());
    }
    Ok(())
}

pub fn dist_binary_cross(target: Targets) -> Result<()> {
    let status = Command::new("cross")
        .current_dir(project_root())
        .args(&[
            "build",
            "--package",
            CRATE_NAME,
            "--target",
            target.into(),
            "--release",
        ])
        .status()?;

    if !status.success() {
        return Err(format!("Cargo build failed with exit code: {:?}", status.code()).into());
    }
    Ok(())
}

pub fn dist_binary(target: Option<Targets>, output: Option<&str>) -> Result<()> {
    let info = os_info::get();
    let os = TryInto::<Targets>::try_into(info.clone())?;
    match target {
        Some(tar) => {
            if os == tar {
                dist_binary_cargo(tar)?;
            } else if (info.os_type() == Type::Macos) && (Into::<Type>::into(tar) == Type::Macos) {
                dist_binary_cargo(tar)?;
            } else {
                dist_binary_cross(tar)?;
            }
        }
        None => {
            dist_binary_cargo(os)?;
        }
    }

    let dst = if let Some(tar) = target {
        project_root().join(format!("target/{}/release/{}", tar, CRATE_NAME))
    } else {
        project_root().join(format!("target/release/{}", CRATE_NAME))
    };
    if !dist_dir(target.map(|t| t.into())).exists() {
        fs::create_dir_all(dist_dir(target.map(|t| t.into())))?;
    }

    let out_dir = match output {
        Some(out) => dist_dir(target.map(|t| t.into()))
            .join(out)
            .join(PACKAGE_NAME),
        None => dist_dir(target.map(|t| t.into())).join(PACKAGE_NAME),
    };

    fs::copy(&dst, out_dir.clone())?;

    Ok(())
}

pub fn dist_readme(target: Option<&str>) -> Result<()> {
    let mut readme = fs::File::create(project_root().join("README.md"))?;
    readme.write_all(clap_markdown::help_markdown::<Cli>().as_bytes())?;
    readme.flush()?;
    drop(readme);

    if dist_dir(target).join("doc").exists() {
        let _ = fs::remove_dir_all(&dist_dir(target).join("doc"));
    }
    fs::create_dir_all(&dist_dir(target).join("doc"))?;

    fs::copy(
        project_root().join("README.md"),
        dist_dir(target).join("doc/README"),
    )?;

    fs::copy(
        project_root().join("README.md"),
        project_root().join(CRATE_NAME).join("README.md"),
    )?;
    Ok(())
}

pub fn dist_changelog(target: Option<&str>) -> Result<()> {
    std::env::set_current_dir(project_root())?;
    let status = Command::new("auto-changelog")
        .stdout(Stdio::null())
        .status()?;

    if !status.success() {
        return Err("Command 'auto-changelog' was unsuccessful"
            .to_string()
            .into());
    }

    fs::copy(
        project_root().join("CHANGELOG.md"),
        dist_dir(target).join("doc/CHANGELOG"),
    )?;

    fs::copy(
        project_root().join("CHANGELOG.md"),
        project_root().join(CRATE_NAME).join("CHANGELOG.md"),
    )?;
    Ok(())
}

pub fn dist_license(target: Option<&str>) -> Result<()> {
    fs::copy(
        project_root().join("LICENSE"),
        dist_dir(target).join("doc/LICENSE"),
    )?;

    fs::copy(
        project_root().join("LICENSE"),
        project_root().join(CRATE_NAME).join("LICENSE"),
    )?;
    Ok(())
}

pub fn dist_manpage(target: Option<&str>) -> Result<()> {
    if dist_dir(target).join("manpages").exists() {
        let _ = fs::remove_dir_all(dist_dir(target).join("manpages"));
    }
    fs::create_dir_all(dist_dir(target).join("manpages"))?;
    clap_mangen::generate_to(Cli::command(), dist_dir(target).join("manpages"))?;
    Ok(())
}
