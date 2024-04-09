use clap::{CommandFactory, Parser};
use std::env::current_dir;
use version_manager::{cli, version::VersionFile, CommandRun, VersionError};

fn main() {
    let args = cli::Cli::parse();

    let curr_dir = match current_dir() {
        Ok(curr_dir) => curr_dir,
        Err(e) => {
            VersionError::IoError(e).terminate(&mut cli::Cli::command());
        }
    };
    let version_file = curr_dir.join("VERSION.toml");

    let mut version = match VersionFile::load(version_file) {
        Ok(version) => version,
        Err(e) => e.terminate(&mut cli::Cli::command()),
    };

    match args.run(&mut version) {
        Ok(_) => match version.save() {
            Ok(_) => (),
            Err(e) => e.terminate(&mut cli::Cli::command()),
        },
        Err(e) => e.terminate(&mut cli::Cli::command()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command() {
        use clap::CommandFactory;
        cli::Cli::command().debug_assert();
    }
}
