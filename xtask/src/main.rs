mod cmd;
use clap::CommandFactory;

use std::{
    env, error, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

type DynError = (String, Box<dyn error::Error>);

const PACKAGE_NAME: &'static str = "version";
const CRATE_NAME: &'static str = "version-manager";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}: {}", e.0, e.1);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task: Vec<String> = env::args().collect();
    if task.len() < 2 {
        print_help()
    } else {
        match task[1].as_str() {
            "dist" => dist()?,
            "target" => target(task[2].clone(), false)?,
            "targets" => targets()?,
            "package" => package(false)?,
            "dist-unk" => dist_unknown()?,
            "package-unk" => package_unknown(false)?,
            "package-targets" => package_targets(false)?,
            "publish-all" => package_targets(true)?,
            "publish" => target(task[2].clone(), true)?,
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
    dist-unk          builds application and man pages for a nonspecific OS
    target [TRIPLE]   builds application and man pages for a specific target
    targets           builds application and man pages for multiple targets
    package           builds and packages a tarball for the application and man pages
    package-unk       builds and packages a tarball for the application and man pages for a nonspecific OS
    package-targets   builds and packages a tarball for the application and man pages for multiple targets
    publish-all       builds, packages, and publishes for multiple targets
    publish [TRIPLE]  builds, packages, and publishes for a specific target


TRIPLES:

    aarch64-apple-darwin
    aarch64-unknown-linux-gnu
    aarch64-unknown-linux-musl
    i686-unknown-linux-gnu
    i686-unknown-linux-musl
    x86_64-apple-darwin
    x86_64-unknown-linux-gnu
    x86_64-unknown-linux-musl
"#
    )
}

static TARGETS: [&'static str; 8] = [
    "aarch64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "aarch64-unknown-linux-musl",
    "i686-unknown-linux-gnu",
    "i686-unknown-linux-musl",
    "x86_64-apple-darwin",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
];

static ARCH: [&'static str; 8] = [
    "aarch64", "aarch64", "aarch64", "i686", "i686", "x86_64", "x86_64", "x86_64",
];

fn targets() -> Result<(), DynError> {
    println!("Installing target build chains");
    install_targets()?;
    build_targets()?;
    Ok(())
}

fn target(target: String, up: bool) -> Result<(), DynError> {
    println!("Installing target build chains");
    if dist_dir(Some(&target)).join(PACKAGE_NAME).exists() {
        if dist_dir(Some(&target)).join(PACKAGE_NAME).is_file() {
            let _ = fs::remove_file(&dist_dir(Some(&target)).join(PACKAGE_NAME));
        }
        let _ = fs::remove_dir_all(&dist_dir(Some(&target)).join(PACKAGE_NAME));
    }
    match fs::create_dir_all(&dist_dir(Some(&target)).join(PACKAGE_NAME)) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                format!("Error when creating 'dist/{}/{}' dir", target, PACKAGE_NAME),
                Box::new(e),
            ))
        }
    };
    let target_dir = format!("{}/{}", target, PACKAGE_NAME);

    install_target(&target)?;
    dist_binary(Some(&target), Some(PACKAGE_NAME))?;
    dist_manpage(Some(&target_dir))?;
    dist_readme(Some(&target_dir))?;
    dist_license(Some(&target_dir))?;
    dist_changelog(Some(&target_dir))?;
    let index = match TARGETS.binary_search(&target.as_str()) {
        Ok(idx) => idx,
        Err(_) => {
            return Err((
                format!("Unable to locate index of {}", &target),
                Box::new(std::fmt::Error),
            ))
        }
    };
    package_release(Some(&target), Some(index), up)?;

    Ok(())
}

fn install_targets() -> Result<(), DynError> {
    let rustup = env::var("RUSTUP").unwrap_or_else(|_| "rustup".to_string());
    for target in TARGETS {
        let status = match Command::new(&rustup)
            .current_dir(project_root())
            .args(&["target", "add", target])
            .status()
        {
            Ok(stat) => stat,
            Err(e) => {
                return Err((
                    format!("Error in command 'cargo target add {}'", target),
                    Box::new(e),
                ))
            }
        };

        if !status.success() {
            return Err((
                format!("failed to add target {}", target),
                Box::new(std::fmt::Error),
            ));
        }
    }

    Ok(())
}

fn install_target(target: &str) -> Result<(), DynError> {
    let rustup = env::var("RUSTUP").unwrap_or_else(|_| "rustup".to_string());
    let status = match Command::new(&rustup)
        .current_dir(project_root())
        .args(&["target", "add", target])
        .status()
    {
        Ok(stat) => stat,
        Err(e) => {
            return Err((
                format!("Error in command 'cargo target add {}'", target),
                Box::new(e),
            ))
        }
    };

    if !status.success() {
        return Err((
            format!("failed to add target {}", target),
            Box::new(std::fmt::Error),
        ));
    }

    Ok(())
}

fn package_targets(up: bool) -> Result<(), DynError> {
    for (i, target) in TARGETS.into_iter().enumerate() {
        if dist_dir(Some(&format!("{}/{}", target, PACKAGE_NAME))).exists() {
            if dist_dir(Some(&format!("{}/{}", target, PACKAGE_NAME))).is_file() {
                let _ = fs::remove_file(&dist_dir(Some(&format!("{}/{}", target, PACKAGE_NAME))));
            }
            let _ = fs::remove_dir_all(&dist_dir(Some(&format!("{}/{}", target, PACKAGE_NAME))));
        }
        match fs::create_dir_all(&dist_dir(Some(&format!("{}/{}", target, PACKAGE_NAME)))) {
            Ok(_) => {}
            Err(e) => {
                return Err((
                    format!(
                        "Error when creating {:?} dir",
                        dist_dir(Some(&format!("{}/{}", target, PACKAGE_NAME)))
                    ),
                    Box::new(e),
                ))
            }
        };

        install_target(&target)?;
        dist_binary(Some(target), Some(PACKAGE_NAME))?;
        dist_manpage(Some(&format!("{}/{}", target, PACKAGE_NAME)))?;
        dist_readme(Some(&format!("{}/{}", target, PACKAGE_NAME)))?;
        dist_license(Some(&format!("{}/{}", target, PACKAGE_NAME)))?;
        dist_changelog(Some(&format!("{}/{}", target, PACKAGE_NAME)))?;

        package_release(Some(target), Some(i), up)?;
    }
    Ok(())
}

fn build_targets() -> Result<(), DynError> {
    if dist_dir(None).exists() {
        let _ = fs::remove_dir_all(&dist_dir(None));
    }
    match fs::create_dir_all(&dist_dir(None)) {
        Ok(_) => {}
        Err(e) => return Err(("Error when creating build dir".to_string(), Box::new(e))),
    };

    for target in TARGETS {
        if dist_dir(Some(target)).exists() {
            let _ = fs::remove_dir_all(&dist_dir(Some(target)));
        }
        match fs::create_dir_all(&dist_dir(Some(target))) {
            Ok(_) => {}
            Err(e) => return Err(("Error when creating target dir".to_string(), Box::new(e))),
        };

        dist_binary(Some(target), None)?;
    }

    Ok(())
}

fn dist_unknown() -> Result<(), DynError> {
    let target = &format!("{}-unknown-none", std::env::consts::ARCH);
    if dist_dir(Some(target)).exists() {
        let _ = fs::remove_dir_all(&dist_dir(Some(target)));
    }
    match fs::create_dir_all(&dist_dir(Some(target))) {
        Ok(_) => {}
        Err(e) => return Err(("Error when creating 'dist' dir".to_string(), Box::new(e))),
    };

    install_target(target)?;
    dist_binary(Some(target), None)?;
    dist_manpage(Some(target))?;
    dist_readme(Some(target))?;
    dist_license(Some(target))?;
    dist_changelog(Some(target))?;

    Ok(())
}

fn package_unknown(up: bool) -> Result<(), DynError> {
    let target = &format!("{}-unknown-none", std::env::consts::ARCH);
    dist_unknown()?;
    let index = match TARGETS.binary_search(&target.as_str()) {
        Ok(idx) => idx,
        Err(_) => {
            return Err((
                format!("Unable to locate index of {}", &target),
                Box::new(std::fmt::Error),
            ))
        }
    };
    package_release(Some(target), Some(index), up)?;
    Ok(())
}

fn dist() -> Result<(), DynError> {
    if dist_dir(None).exists() {
        let _ = fs::remove_dir_all(&dist_dir(None));
    }
    match fs::create_dir_all(&dist_dir(None)) {
        Ok(_) => {}
        Err(e) => return Err(("Error when creating 'dist' dir".to_string(), Box::new(e))),
    };

    dist_binary(None, None)?;
    dist_manpage(None)?;
    dist_readme(None)?;
    dist_license(None)?;
    dist_changelog(None)?;

    Ok(())
}

fn package(up: bool) -> Result<(), DynError> {
    if dist_dir(Some(PACKAGE_NAME)).exists() {
        if dist_dir(Some(PACKAGE_NAME)).is_file() {
            let _ = fs::remove_file(&dist_dir(Some(PACKAGE_NAME)));
        }
        let _ = fs::remove_dir_all(&dist_dir(Some(PACKAGE_NAME)));
    }
    match fs::create_dir_all(&dist_dir(Some(PACKAGE_NAME))) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                format!("Error when creating 'dist/{}' dir", PACKAGE_NAME),
                Box::new(e),
            ))
        }
    };

    dist_binary(None, Some(PACKAGE_NAME))?;
    dist_manpage(Some(PACKAGE_NAME))?;
    dist_readme(Some(PACKAGE_NAME))?;
    dist_license(Some(PACKAGE_NAME))?;
    dist_changelog(Some(PACKAGE_NAME))?;

    package_release(None, None, up)?;
    Ok(())
}

fn dist_binary(target: Option<&str>, output: Option<&str>) -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = match target {
        Some(tar) => match Command::new(cargo)
            .current_dir(project_root())
            .args(&[
                "build",
                "--package",
                CRATE_NAME,
                "--target",
                tar,
                "--release",
            ])
            .status()
        {
            Ok(stat) => stat,
            Err(e) => {
                return Err((
                    format!(
                        "Error in command 'cargo build --package {} --target {} --release'",
                        CRATE_NAME, tar
                    ),
                    Box::new(e),
                ))
            }
        },
        None => match Command::new(cargo)
            .current_dir(project_root())
            .args(&["build", "--release"])
            .status()
        {
            Ok(stat) => stat,
            Err(e) => {
                return Err((
                    "Error in command 'cargo build --release'".to_string(),
                    Box::new(e),
                ))
            }
        },
    };

    if !status.success() {
        return Err((
            format!("Cargo build failed with exit code: {:?}", status.code()),
            Box::new(std::fmt::Error),
        ));
    }

    let dst = if let Some(tar) = target {
        project_root().join(format!("target/{}/release/{}", tar, CRATE_NAME))
    } else {
        project_root().join(format!("target/release/{}", CRATE_NAME))
    };
    if !dist_dir(target).exists() {
        match fs::create_dir_all(dist_dir(target)) {
            Ok(_) => {}
            Err(e) => {
                return Err((
                    format!(
                        "[355] Error when creating subdirectory {:?}",
                        dist_dir(target)
                    ),
                    Box::new(e),
                ))
            }
        };
    }

    let out_dir = match output {
        Some(out) => dist_dir(target).join(out).join(PACKAGE_NAME),
        None => dist_dir(target).join(PACKAGE_NAME),
    };

    match fs::copy(&dst, out_dir.clone()) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                format!("[371] Error when copying {:?} to {:?}", dst, out_dir),
                Box::new(e),
            ))
        }
    };

    Ok(())
}

fn dist_readme(target: Option<&str>) -> Result<(), DynError> {
    let mut readme = match fs::File::create(project_root().join("README.md")) {
        Ok(re) => re,
        Err(e) => {
            return Err((
                "Error when trying to create the README".to_string(),
                Box::new(e),
            ))
        }
    };
    match readme.write_all(clap_markdown::help_markdown::<cmd::Cli>().as_bytes()) {
        Ok(_) => {}
        Err(e) => return Err(("Error when writing to the README".to_string(), Box::new(e))),
    };
    match readme.flush() {
        Ok(_) => {}
        Err(e) => {
            return Err((
                "Unable to flush the README file buffer".to_string(),
                Box::new(e),
            ))
        }
    };

    if dist_dir(target).join("doc").exists() {
        let _ = fs::remove_dir_all(&dist_dir(target).join("doc"));
    }
    match fs::create_dir_all(&dist_dir(target).join("doc")) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                "Error when creating 'dist/doc' dir".to_string(),
                Box::new(e),
            ))
        }
    };

    match fs::copy(
        project_root().join("README.md"),
        dist_dir(target).join("doc/README"),
    ) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                format!(
                    "Error when copying {:?} to {:?}",
                    project_root().join("README.md"),
                    dist_dir(target).join("doc/README"),
                ),
                Box::new(e),
            ))
        }
    };

    match fs::copy(
        project_root().join("README.md"),
        project_root().join(CRATE_NAME).join("README.md"),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err((
            format!(
                "Error when copying {:?} to {:?}",
                project_root().join("README.md"),
                project_root().join(CRATE_NAME).join("README.md"),
            ),
            Box::new(e),
        )),
    }
}

fn dist_changelog(target: Option<&str>) -> Result<(), DynError> {
    match std::env::set_current_dir(project_root()) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                "Unable to cd into the project root".to_string(),
                Box::new(e),
            ))
        }
    };
    let status = match Command::new("auto-changelog")
        .stdout(Stdio::null())
        .status()
    {
        Ok(stat) => stat,
        Err(e) => {
            return Err((
                "Error when running command 'auto-changelog'".to_string(),
                Box::new(e),
            ))
        }
    };

    if !status.success() {
        return Err((
            "Command 'auto-changelog' was unsuccessful".to_string(),
            Box::new(std::fmt::Error),
        ));
    }

    match fs::copy(
        project_root().join("CHANGELOG.md"),
        dist_dir(target).join("doc/CHANGELOG"),
    ) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                format!(
                    "Error when copying {:?} to {:?}",
                    project_root().join("CHANGELOG.md"),
                    dist_dir(target).join("doc/CHANGELOG"),
                ),
                Box::new(e),
            ))
        }
    };

    match fs::copy(
        project_root().join("CHANGELOG.md"),
        project_root().join(CRATE_NAME).join("CHANGELOG.md"),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err((
            format!(
                "Error when copying {:?} to {:?}",
                project_root().join("CHANGELOG.md"),
                project_root().join(CRATE_NAME).join("CHANGELOG.md"),
            ),
            Box::new(e),
        )),
    }
}

fn dist_license(target: Option<&str>) -> Result<(), DynError> {
    match fs::copy(
        project_root().join("LICENSE"),
        dist_dir(target).join("doc/LICENSE"),
    ) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                format!(
                    "Error when copying {:?} to {:?}",
                    project_root().join("LICENSE"),
                    dist_dir(target).join("doc/LICENSE"),
                ),
                Box::new(e),
            ))
        }
    };

    match fs::copy(
        project_root().join("LICENSE"),
        project_root().join(CRATE_NAME).join("LICENSE"),
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err((
            format!(
                "Error when copying {:?} to {:?}",
                project_root().join("LICENSE"),
                project_root().join(CRATE_NAME).join("LICENSE"),
            ),
            Box::new(e),
        )),
    }
}

fn dist_manpage(target: Option<&str>) -> Result<(), DynError> {
    if dist_dir(target).join("manpages").exists() {
        let _ = fs::remove_dir_all(dist_dir(target).join("manpages"));
    }
    match fs::create_dir_all(dist_dir(target).join("manpages")) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                format!(
                    "Unable to create directory {:?}",
                    dist_dir(target).join("manpages")
                ),
                Box::new(e),
            ))
        }
    }
    match clap_mangen::generate_to(cmd::Cli::command(), dist_dir(target).join("manpages")) {
        Ok(_) => {}
        Err(e) => return Err(("Error when creating the MAN pages".to_string(), Box::new(e))),
    };
    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn dist_dir(target: Option<&str>) -> PathBuf {
    match target {
        Some(target) => project_root().join(format!("target/dist/{}", target)),
        None => project_root().join("target/dist"),
    }
}

fn get_host() -> Result<String, DynError> {
    let output = match Command::new("rustc").arg("-vV").output() {
        Ok(host) => host,
        Err(e) => return Err(("Unable to get rustc output".to_string(), Box::new(e))),
    };

    let stdout_buf = match String::from_utf8(output.stdout) {
        Ok(out) => out,
        Err(e) => return Err(("Unable to parse output".to_string(), Box::new(e))),
    };
    match stdout_buf.lines().find_map(|l| l.strip_prefix("host: ")) {
        Some(host) => Ok(host.to_string()),
        None => Err((
            "Host triple not found".to_string(),
            Box::new(std::fmt::Error),
        )),
    }
}

fn generate_rpm(target: Option<usize>) -> Result<(), DynError> {
    let status = match target {
        Some(tgt) => match Command::new("cargo")
            .current_dir(project_root())
            .arg("generate-rpm")
            .arg("--arch")
            .arg(ARCH[tgt])
            .arg("--package")
            .arg(CRATE_NAME)
            .arg("--output")
            .arg(format!(
                "{}/{}-v{}-{}.rpm",
                dist_dir(Some(&TARGETS[tgt])).to_string_lossy(),
                PACKAGE_NAME,
                env!("CARGO_PKG_VERSION"),
                &TARGETS[tgt]
            ))
            .stdout(Stdio::null())
            .status()
        {
            Ok(stat) => stat,
            Err(e) => {
                return Err((
                    format!(
                        "Failed when running 'cargo generate-rpm --arch {} --package {} --output {}/{}-v{}-{}.rpm'",
                        ARCH[tgt], CRATE_NAME,
                        dist_dir(Some(&TARGETS[tgt])).to_string_lossy(),
                        PACKAGE_NAME,
                        env!("CARGO_PKG_VERSION"),
                        &TARGETS[tgt]
                    ),
                    Box::new(e),
                ))
            }
        },
        None => match Command::new("cargo")
            .current_dir(project_root())
            .arg("generate-rpm")
            .arg("--package")
            .arg(CRATE_NAME)
            .stdout(Stdio::null())
            .status()
        {
            Ok(stat) => stat,
            Err(e) => {
                return Err((
                    format!(
                        "Failed when running 'cargo generate-rpm --package {}'",
                        CRATE_NAME
                    ),
                    Box::new(e),
                ))
            }
        },
    };

    if !status.success() {
        return Err((
            "Command 'cargo generate-rpm ...' exited unsuccessfully".to_string(),
            Box::new(std::fmt::Error),
        ));
    }
    Ok(())
}

fn generate_deb(target: Option<&str>) -> Result<(), DynError> {
    let status = match target {
        Some(tgt) => match Command::new("cargo")
            .current_dir(project_root())
            .arg("deb")
            .arg("--target")
            .arg(tgt)
            .arg("--package")
            .arg(CRATE_NAME)
            .arg("--no-build")
            .arg("--output")
            .arg(format!(
                "{}/{}-v{}-{}.deb",
                dist_dir(target).to_string_lossy(),
                PACKAGE_NAME,
                env!("CARGO_PKG_VERSION"),
                tgt
            ))
            .stdout(Stdio::null())
            .status()
        {
            Ok(stat) => stat,
            Err(e) => {
                return Err((
                    format!(
                        "Failed when running 'cargo deb --target {} --package {} --no-build --output {}/{}-v{}-{}.deb'",
                        tgt, CRATE_NAME,
                        dist_dir(target).to_string_lossy(),
                        PACKAGE_NAME,
                        env!("CARGO_PKG_VERSION"),
                        tgt
                    ),
                    Box::new(e),
                ))
            }
        },
        None => match Command::new("cargo")
            .current_dir(project_root())
            .arg("deb")
            .arg("--package")
            .arg(CRATE_NAME)
            .arg("--no-build")
            .stdout(Stdio::null())
            .status()
        {
            Ok(stat) => stat,
            Err(e) => {
                return Err((
                    format!(
                        "Failed when running 'cargo deb --package {} --no-build'",
                        CRATE_NAME
                    ),
                    Box::new(e),
                ))
            }
        },
    };

    if !status.success() {
        return Err((
            "Command 'cargo deb ...' exited unsuccessfully".to_string(),
            Box::new(std::fmt::Error),
        ));
    }
    Ok(())
}

fn package_release(target: Option<&str>, index: Option<usize>, up: bool) -> Result<(), DynError> {
    let version = env!("CARGO_PKG_VERSION");
    let host = match target {
        Some(target) => target.to_owned(),
        None => get_host()?,
    };

    let package = format!("{}-v{}-{}.tar.gz", PACKAGE_NAME, version, host);
    let package_rpm = format!("{}-v{}-{}.rpm", PACKAGE_NAME, version, host);
    let package_deb = format!("{}-v{}-{}.deb", PACKAGE_NAME, version, host);

    match std::env::set_current_dir(dist_dir(target)) {
        Ok(_) => {}
        Err(e) => {
            return Err((
                "Unable to cd into the dist directory".to_string(),
                Box::new(e),
            ))
        }
    };

    let status = match Command::new("tar")
        .arg("-c")
        .arg("-z")
        .arg("-f")
        .arg(package.clone())
        .arg(PACKAGE_NAME)
        .stdout(Stdio::null())
        .status()
    {
        Ok(stat) => stat,
        Err(e) => {
            return Err((
                format!(
                    "Error when running command 'tar -c -z -f {} {}'",
                    package, PACKAGE_NAME
                ),
                Box::new(e),
            ))
        }
    };

    if !status.success() {
        return Err((
            format!(
                "Command 'tar -c -z -f {} {}' was unsuccessful",
                package, PACKAGE_NAME
            ),
            Box::new(std::fmt::Error),
        ));
    }

    if let Some(tar) = target {
        match fs_extra::dir::copy(
            dist_dir(None).join(format!("{}/{}", tar, PACKAGE_NAME)),
            dist_dir(None),
            &fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .skip_exist(false)
                .copy_inside(false)
                .content_only(true),
        ) {
            Ok(_) => {}
            Err(e) => {
                return Err((
                    format!(
                        "Error when copying {:?} to {:?}",
                        dist_dir(None).join(format!("{}/{}", tar, PACKAGE_NAME)),
                        dist_dir(None)
                    ),
                    Box::new(e),
                ))
            }
        }
    }

    generate_rpm(index)?;
    generate_deb(target)?;

    if up {
        match target {
            Some(tgt) => publish(
                version.to_string(),
                format!("{}/{}", dist_dir(Some(tgt)).to_string_lossy(), package_rpm),
            )?,
            None => publish(
                version.to_string(),
                format!(
                    "{}/*.rpm",
                    dist_dir(None).join("generate-rpm").to_string_lossy()
                ),
            )?,
        }
        match target {
            Some(tgt) => publish(
                version.to_string(),
                format!("{}/{}", dist_dir(Some(tgt)).to_string_lossy(), package_deb),
            )?,
            None => publish(
                version.to_string(),
                format!("{}/*.deb", dist_dir(None).join("debian").to_string_lossy()),
            )?,
        }
        publish(version.to_string(), package)?;
    }

    Ok(())
}

fn publish(version: String, file: String) -> Result<(), DynError> {
    let status = match Command::new("gh")
        .arg("release")
        .arg("upload")
        .arg("--clobber")
        .arg(format!("v{}", version))
        .arg(file.clone())
        .stdout(Stdio::null())
        .status()
    {
        Ok(stat) => stat,
        Err(e) => {
            return Err((
                format!(
                    "Error when running 'gh release upload v{} {}'",
                    version, file
                ),
                Box::new(e),
            ))
        }
    };

    if !status.success() {
        return Err((
            format!(
                "Command 'gh release upload v{} {}' was unsuccessful",
                version, file
            ),
            Box::new(std::fmt::Error),
        ));
    }
    Ok(())
}
