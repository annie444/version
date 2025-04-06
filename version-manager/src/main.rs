use clap::{CommandFactory, Parser};
use version_manager::{VersionError, cli, run};

use std::env::current_dir;
fn main() {
    let mut args = cli::Cli::parse();

    let curr_dir = match current_dir() {
        Ok(curr_dir) => curr_dir,
        Err(e) => {
            VersionError::IoError(e).terminate(&mut cli::Cli::command());
        }
    };
    let version_file = curr_dir.join("VERSION.toml");

    match args.run() {
        Ok(scope) => match scope {
            Some(scope) => match run::run(scope, version_file) {
                Ok(_) => {}
                Err(e) => e.terminate(&mut cli::Cli::command()),
            },
            None => (),
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
