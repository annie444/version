use clap::{
    builder::RangedI64ValueParser, value_parser, Args, Command, CommandFactory, Parser, Subcommand,
};
use clap_complete::{generate, Generator, Shell};
use serde::{Deserialize, Serialize};
use std::{env, fs::read_to_string, io};

#[derive(Serialize, Deserialize)]
struct Version {
    version: String,
    major: i8,
    minor: i8,
    patch: i8,
    alpha: Option<i8>,
    beta: Option<i8>,
    rc: Option<i8>,
    build: String,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: VersionCommand,
    #[arg(value_parser = value_parser!(Shell), exclusive = true)]
    generator: Option<Shell>,
}

#[derive(Subcommand)]
/// Specify the version number to change
#[command(rename_all = "lower")]
enum VersionCommand {
    Major(GetSet),
    Minor(GetSet),
    Patch(GetSet),
    Alpha(GetSetRm),
    Beta(GetSetRm),
    RC(GetSetRm),
    Build(GetSetCommit),
    Get,
}

#[derive(Parser)]
/// Get or set the version number
struct GetSet {
    #[command(subcommand)]
    command: GetSetCommand,
}

#[derive(Subcommand)]
/// Get or set the version number
enum GetSetCommand {
    Get,
    Set(Set),
}

#[derive(Parser)]
/// Get or set the version number
struct GetSetRm {
    #[command(subcommand)]
    command: GetSetRmCommand,
}

#[derive(Subcommand)]
/// Get or set the version number
enum GetSetRmCommand {
    Get,
    Set(Set),
    Rm,
}

#[derive(Args)]
/// Set the version number
struct Set {
    #[arg(value_parser = RangedI64ValueParser::<i8>::new(), exclusive = true)]
    value: Option<i8>,
    #[command(subcommand)]
    command: Option<UpDown>,
}

#[derive(Subcommand)]
#[command(rename_all = "lower")]
enum UpDown {
    #[command(name = "+")]
    Up,
    #[command(name = "-")]
    Down,
}

#[derive(Parser)]
/// Get or set the build version
struct GetSetCommit {
    #[command(subcommand)]
    command: GetSetCommitCommand,
}

#[derive(Subcommand)]
/// Get or set the build version
enum GetSetCommitCommand {
    Get,
    Set(SetCommit),
    Rm,
}

#[derive(Args)]
/// Set the build version
struct SetCommit {
    #[arg()]
    value: String,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

fn get_or_set(gs: GetSet, parent: &str) {
    match gs.command {
        GetSetCommand::Get => {
            println!("Get {}", parent);
        }
        GetSetCommand::Set(s) => match parent {
            "build" => set_commit(s),

            _ => set(s, parent),
        },
    }
}

fn get_version() -> Version {
    let curr_dir = env::current_dir().expect("Unable to get current directory");
    let version_file = curr_dir.join("VERSION");
    if version_file.exists() {
        let version = read_to_string(version_file).expect("Unable to read VERSION file");
        toml::from_str(&version).unwrap()
    } else {
        Version {
            version: "0.1.0".to_string(),
            major: 0,
            minor: 1,
            patch: 0,
            alpha: None,
            beta: None,
            rc: None,
            build: "".to_string(),
        }
    }
}

fn change_value(val: i8, s: Set) -> i8 {
    if let Some(cmd) = s.command {
        match cmd {
            UpDown::Up => val + 1,
            UpDown::Down => val - 1,
        }
    } else if let Some(value) = s.value {
        val + value
    } else {
        panic!("Not a valid set command");
    }
}

fn set_commit()

fn set(s: set, parent: &str) {
    let mut version = get_version();
    match parent {
        "major" => {
            version.major = change_value(version.major, s);
        }
        "minor" => {
            version.minor = change_value(version.minor, s);
        }
        "patch" => {
            version.patch = change_value(version.minor, s);
        }
        "alpha" => {
            version.alpha = some(change_value(version.minor, s));
        }
        "beta" => {
            version.beta = some(change_value(version.minor, s));
        }
        "rc" => {
            version.rc = some(change_value(version.minor, s));
        }
        _ => {
            println!("unknown version number");
            return;
        }
    }
    let curr_dir = env::current_dir().expect("unable to get current directory");
    let version_file = curr_dir.join("version");
    let version_str = toml::to_string(&version).unwrap();
    std::fs::write(version_file, version_str).expect("unable to write version file");
}

fn main() {
    let args = Cli::parse();

    if let Some(generator) = args.generator {
        let mut cmd = Cli::command();
        print_completions(generator, &mut cmd);
        return;
    }

    match args.command {
        VersionCommand::Major(cmd) => {
            get_or_set(cmd, "major");
        }
        VersionCommand::Minor(_) => {
            println!("Minor version");
        }
        VersionCommand::Patch(_) => {
            println!("Patch version");
        }
        VersionCommand::Alpha(_) => {
            println!("Alpha version");
        }
        VersionCommand::Beta(_) => {
            println!("Beta version");
        }
        VersionCommand::RC(_) => {
            println!("RC version");
        }
        VersionCommand::Build(_) => {
            println!("Build version");
        }
        VersionCommand::Get => {
            println!("Get version");
        }
    }
}
