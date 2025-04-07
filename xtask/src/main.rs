use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
use strum::IntoEnumIterator;

use targets::Targets;

pub mod dist;
pub mod error;
pub mod targets;

use error::Result;

use self::targets::Arch;

const PACKAGE_NAME: &'static str = "version";
const CRATE_NAME: &'static str = "version-manager";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<()> {
    let task: Vec<String> = env::args().collect();
    if task.len() < 2 {
        print_help()
    } else {
        match task[1].as_str() {
            "dist" => dist::dist()?,
            "target" => target(task[2].clone().try_into()?, false)?,
            "targets" => targets()?,
            "package" => package(false)?,
            "package-targets" => package_targets(false)?,
            "publish-all" => package_targets(true)?,
            "publish" => target(task[2].clone().try_into()?, true)?,
            _ => print_help(),
        }
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"
COMMANDS:

    dist              builds application and man pages
    target [TRIPLE]   builds application and man pages for a specific target
    targets           builds application and man pages for multiple targets
    package           builds and packages a tarball for the application and man pages
    package-targets   builds and packages a tarball for the application and man pages for multiple targets
    publish-all       builds, packages, and publishes for multiple targets
    publish [TRIPLE]  builds, packages, and publishes for a specific target


TRIPLES:

"#
    );
    for target in Targets::iter() {
        eprintln!("    {}", target);
    }
}

fn targets() -> Result<()> {
    println!("Installing target build chains");
    install_targets()?;
    build_targets()?;
    Ok(())
}

fn target(target: Targets, up: bool) -> Result<()> {
    println!("Installing target build chains");
    if dist::dist_dir(Some(&target.to_string()))
        .join(PACKAGE_NAME)
        .exists()
    {
        if dist::dist_dir(Some(&target.to_string()))
            .join(PACKAGE_NAME)
            .is_file()
        {
            let _ = fs::remove_file(&dist::dist_dir(Some(target.into())).join(PACKAGE_NAME));
        }
        let _ = fs::remove_dir_all(&dist::dist_dir(Some(target.into())).join(PACKAGE_NAME));
    }
    fs::create_dir_all(&dist::dist_dir(Some(target.into())).join(PACKAGE_NAME))?;
    let target_dir = format!("{}/{}", target, PACKAGE_NAME);

    install_target(target)?;
    dist::dist_binary(Some(target), Some(PACKAGE_NAME))?;
    dist::dist_manpage(Some(&target_dir))?;
    dist::dist_readme(Some(&target_dir))?;
    dist::dist_license(Some(&target_dir))?;
    dist::dist_changelog(Some(&target_dir))?;
    package_release(Some(target), up)?;

    Ok(())
}

fn install_targets() -> Result<()> {
    let rustup = env::var("RUSTUP").unwrap_or_else(|_| "rustup".to_string());
    for target in Targets::iter() {
        let status = Command::new(&rustup)
            .current_dir(project_root())
            .args(&["target", "add", target.into()])
            .status()?;

        if !status.success() {
            return Err(format!("failed to add target {}", target).into());
        }
    }

    Ok(())
}

fn install_target(target: Targets) -> Result<()> {
    let rustup = env::var("RUSTUP").unwrap_or_else(|_| "rustup".to_string());
    let status = Command::new(&rustup)
        .current_dir(project_root())
        .args(&["target", "add", target.into()])
        .status()?;

    if !status.success() {
        return Err(format!("failed to add target {}", target).into());
    }

    Ok(())
}

fn package_targets(up: bool) -> Result<()> {
    for target in Targets::iter() {
        if dist::dist_dir(Some(&format!("{}/{}", target, PACKAGE_NAME))).exists() {
            if dist::dist_dir(Some(&format!("{}/{}", target, PACKAGE_NAME))).is_file() {
                let _ = fs::remove_file(&dist::dist_dir(Some(&format!(
                    "{}/{}",
                    target, PACKAGE_NAME
                ))));
            }
            let _ = fs::remove_dir_all(&dist::dist_dir(Some(&format!(
                "{}/{}",
                target, PACKAGE_NAME
            ))));
        }
        fs::create_dir_all(&dist::dist_dir(Some(&format!(
            "{}/{}",
            target, PACKAGE_NAME
        ))))?;

        install_target(target)?;
        dist::dist_binary(Some(target.into()), Some(PACKAGE_NAME))?;
        dist::dist_manpage(Some(&format!("{}/{}", target, PACKAGE_NAME)))?;
        dist::dist_readme(Some(&format!("{}/{}", target, PACKAGE_NAME)))?;
        dist::dist_license(Some(&format!("{}/{}", target, PACKAGE_NAME)))?;
        dist::dist_changelog(Some(&format!("{}/{}", target, PACKAGE_NAME)))?;

        package_release(Some(target), up)?;
    }
    Ok(())
}

fn build_targets() -> Result<()> {
    if dist::dist_dir(None).exists() {
        let _ = fs::remove_dir_all(&dist::dist_dir(None));
    }
    fs::create_dir_all(&dist::dist_dir(None))?;

    for target in Targets::iter() {
        if dist::dist_dir(Some(target.into())).exists() {
            let _ = fs::remove_dir_all(&dist::dist_dir(Some(target.into())));
        }
        fs::create_dir_all(&dist::dist_dir(Some(target.into())))?;

        dist::dist_binary(Some(target), None)?;
    }

    Ok(())
}

fn package(up: bool) -> Result<()> {
    if dist::dist_dir(Some(PACKAGE_NAME)).exists() {
        if dist::dist_dir(Some(PACKAGE_NAME)).is_file() {
            let _ = fs::remove_file(&dist::dist_dir(Some(PACKAGE_NAME)));
        }
        let _ = fs::remove_dir_all(&dist::dist_dir(Some(PACKAGE_NAME)));
    }
    fs::create_dir_all(&dist::dist_dir(Some(PACKAGE_NAME)))?;

    dist::dist_binary(None, Some(PACKAGE_NAME))?;
    dist::dist_manpage(Some(PACKAGE_NAME))?;
    dist::dist_readme(Some(PACKAGE_NAME))?;
    dist::dist_license(Some(PACKAGE_NAME))?;
    dist::dist_changelog(Some(PACKAGE_NAME))?;

    package_release(None, up)?;
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn get_host() -> Result<Targets> {
    let target: Targets = os_info::get().try_into()?;
    Ok(target)
}

fn generate_rpm(target: Option<Targets>) -> Result<()> {
    let status = match target {
        Some(tgt) => Command::new("cargo")
            .current_dir(project_root())
            .arg("generate-rpm")
            .arg("--auto-req")
            .arg("disabled")
            .arg("--arch")
            .arg(Into::<Arch>::into(tgt).to_string())
            .arg("--package")
            .arg(CRATE_NAME)
            .arg("--output")
            .arg(format!(
                "{}/{}-v{}-{}.rpm",
                dist::dist_dir(Some(&tgt.to_string())).to_string_lossy(),
                PACKAGE_NAME,
                env!("CARGO_PKG_VERSION"),
                &tgt
            ))
            .stdout(Stdio::null())
            .status()?,
        None => Command::new("cargo")
            .current_dir(project_root())
            .arg("generate-rpm")
            .arg("--package")
            .arg(CRATE_NAME)
            .stdout(Stdio::null())
            .status()?,
    };

    if !status.success() {
        return Err("Command 'cargo generate-rpm ...' exited unsuccessfully"
            .to_string()
            .into());
    }
    Ok(())
}

fn generate_deb(target: Option<Targets>) -> Result<()> {
    let status = match target {
        Some(tgt) => Command::new("cargo")
            .current_dir(project_root())
            .arg("deb")
            .arg("--target")
            .arg(tgt.to_string())
            .arg("--package")
            .arg(CRATE_NAME)
            .arg("--no-build")
            .arg("--output")
            .arg(format!(
                "{}/{}-v{}-{}.deb",
                dist::dist_dir(target.map(|t| t.into())).to_string_lossy(),
                PACKAGE_NAME,
                env!("CARGO_PKG_VERSION"),
                tgt
            ))
            .stdout(Stdio::null())
            .status()?,
        None => Command::new("cargo")
            .current_dir(project_root())
            .arg("deb")
            .arg("--package")
            .arg(CRATE_NAME)
            .arg("--no-build")
            .stdout(Stdio::null())
            .status()?,
    };

    if !status.success() {
        return Err("Command 'cargo deb ...' exited unsuccessfully"
            .to_string()
            .into());
    }
    Ok(())
}

fn package_release(target: Option<Targets>, up: bool) -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let host = match target {
        Some(target) => target.to_owned(),
        None => get_host()?,
    };

    let package = format!("{}-v{}-{}.tar.gz", PACKAGE_NAME, version, host);
    let package_rpm = format!("{}-v{}-{}.rpm", PACKAGE_NAME, version, host);
    let package_deb = format!("{}-v{}-{}.deb", PACKAGE_NAME, version, host);

    std::env::set_current_dir(dist::dist_dir(target.map(|t| t.into())))?;

    let status = Command::new("tar")
        .arg("-c")
        .arg("-z")
        .arg("-f")
        .arg(package.clone())
        .arg(PACKAGE_NAME)
        .stdout(Stdio::null())
        .status()?;

    if !status.success() {
        return Err(format!(
            "Command 'tar -c -z -f {} {}' was unsuccessful",
            package, PACKAGE_NAME
        )
        .into());
    }

    if let Some(tar) = target {
        fs_extra::dir::copy(
            dist::dist_dir(None).join(format!("{}/{}", tar, PACKAGE_NAME)),
            dist::dist_dir(None),
            &fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .skip_exist(false)
                .copy_inside(false)
                .content_only(true),
        )?;
    }

    generate_rpm(target)?;
    generate_deb(target)?;

    if up {
        match target {
            Some(tgt) => publish(
                version.to_string(),
                format!(
                    "{}/{}",
                    dist::dist_dir(Some(tgt.into())).to_string_lossy(),
                    package_rpm
                ),
            )?,
            None => publish(
                version.to_string(),
                format!(
                    "{}/*.rpm",
                    dist::dist_dir(None).join("generate-rpm").to_string_lossy()
                ),
            )?,
        }
        match target {
            Some(tgt) => publish(
                version.to_string(),
                format!(
                    "{}/{}",
                    dist::dist_dir(Some(tgt.into())).to_string_lossy(),
                    package_deb
                ),
            )?,
            None => publish(
                version.to_string(),
                format!(
                    "{}/*.deb",
                    dist::dist_dir(None).join("debian").to_string_lossy()
                ),
            )?,
        }
        publish(version.to_string(), package)?;
    }

    Ok(())
}

fn publish(version: String, file: String) -> Result<()> {
    let status = Command::new("gh")
        .arg("release")
        .arg("upload")
        .arg("--clobber")
        .arg(format!("v{}", version))
        .arg(file.clone())
        .stdout(Stdio::null())
        .status()?;

    if !status.success() {
        return Err(format!(
            "Command 'gh release upload v{} {}' was unsuccessful",
            version, file
        )
        .into());
    }
    Ok(())
}
