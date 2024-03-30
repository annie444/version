use ::version::{cli, version::VersionFile};
use clap::{CommandFactory, Parser};

fn main() {
    let app = cli::Cli::parse();

    let mut version = match VersionFile::load() {
        Ok(version) => version,
        Err(e) => e.terminate(&mut cli::Cli::command()),
    };

    match app.run(&mut version) {
        Ok(_) => match version.save() {
            Ok(_) => (),
            Err(e) => e.terminate(&mut cli::Cli::command()),
        },
        Err(e) => e.terminate(&mut cli::Cli::command()),
    }
}
