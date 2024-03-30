mod cmd;
use clap::CommandFactory;

use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist()?,
        Some("target") => target()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

dist            builds application and man pages
target          builds application and man pages for multiple targets
"
    )
}

static TARGETS: [&'static str; 13] = [
    "aarch64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "aarch64-unknown-linux-musl",
    "aarch64-unknown-none",
    "aarch64-unknown-uefi",
    "x86_64-apple-darwin",
    "x86_64-sun-solaris",
    "x86_64-unknown-freebsd",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "x86_64-unknown-netbsd",
    "x86_64-unknown-none",
    "x86_64-unknown-uefi",
];

fn target() -> Result<(), DynError> {
    println!("Installing target build chains");
    install_targets()?;
    build_targets()?;
    Ok(())
}

fn install_targets() -> Result<(), DynError> {
    let rustup = env::var("RUSTUP").unwrap_or_else(|_| "rustup".to_string());
    for target in TARGETS {
        let status = Command::new(&rustup)
            .current_dir(project_root())
            .args(&["target", "add", target])
            .status()?;

        if !status.success() {
            Err(format!("failed to add target {}", target))?;
        }
    }

    Ok(())
}

fn build_targets() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_dir(None));
    fs::create_dir_all(&dist_dir(None))?;

    for target in TARGETS {
        let _ = fs::remove_dir_all(&dist_dir(Some(target)));
        fs::create_dir_all(&dist_dir(Some(target)))?;

        dist_binary(Some(target))?;
    }

    Ok(())
}

fn dist() -> Result<(), DynError> {
    let _ = fs::remove_dir_all(&dist_dir(None));
    fs::create_dir_all(&dist_dir(None))?;

    dist_binary(None)?;
    dist_manpage()?;

    Ok(())
}

fn dist_binary(target: Option<&str>) -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = match target {
        Some(tar) => Command::new(cargo)
            .current_dir(project_root())
            .args(&["build", "--target", tar, "--release"])
            .status()?,
        None => Command::new(cargo)
            .current_dir(project_root())
            .args(&["build", "--release"])
            .status()?,
    };

    if !status.success() {
        Err("cargo build failed")?;
    }

    let dst = project_root().join("target/release/version");

    fs::copy(&dst, dist_dir(target).join("version"))?;

    if Command::new("strip")
        .arg("--version")
        .stdout(Stdio::null())
        .status()
        .is_ok()
    {
        eprintln!("stripping the binary");
        let status = Command::new("strip")
            .arg("--strip-all")
            .arg(&dst)
            .status()?;
        if !status.success() {
            Err("strip failed")?;
        }
    } else {
        eprintln!("no `strip` utility found")
    }

    let mut readme = fs::File::create(env::current_dir()?.join("README.md"))?;
    readme.write_all(clap_markdown::help_markdown::<cmd::Cli>().as_bytes())?;
    readme.flush()?;

    Ok(())
}

fn dist_manpage() -> Result<(), DynError> {
    clap_mangen::generate_to(cmd::Cli::command(), dist_dir(None))?;
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
