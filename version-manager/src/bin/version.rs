use clap::{CommandFactory, Parser};
use version_manager::{cli, version::VersionFile};

fn main() {
    let args = cli::Cli::parse();

    let mut version = match VersionFile::load() {
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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
