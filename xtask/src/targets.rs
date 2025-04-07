use crate::error::XtaskError;
use os_info::{Bitness, Info, Type};
use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter)]
pub enum Targets {
    Aarch64AppleDarwin,
    Aarch64UnknownLinuxGnu,
    Aarch64UnknownLinuxMusl,
    I686UnknownLinuxGnu,
    I686UnknownLinuxMusl,
    X8664AppleDarwin,
    X8664UnknownLinuxGnu,
    X8664UnknownLinuxMusl,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter)]
pub enum Arch {
    Aarch64,
    I686,
    X8664,
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Arch::Aarch64 => write!(f, "aarch64"),
            Arch::I686 => write!(f, "i686"),
            Arch::X8664 => write!(f, "x86_64"),
        }
    }
}

impl From<Targets> for Arch {
    fn from(target: Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => Arch::Aarch64,
            Targets::Aarch64UnknownLinuxGnu => Arch::Aarch64,
            Targets::Aarch64UnknownLinuxMusl => Arch::Aarch64,
            Targets::I686UnknownLinuxGnu => Arch::I686,
            Targets::I686UnknownLinuxMusl => Arch::I686,
            Targets::X8664AppleDarwin => Arch::X8664,
            Targets::X8664UnknownLinuxGnu => Arch::X8664,
            Targets::X8664UnknownLinuxMusl => Arch::X8664,
        }
    }
}

impl From<&Targets> for Arch {
    fn from(target: &Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => Arch::Aarch64,
            Targets::Aarch64UnknownLinuxGnu => Arch::Aarch64,
            Targets::Aarch64UnknownLinuxMusl => Arch::Aarch64,
            Targets::I686UnknownLinuxGnu => Arch::I686,
            Targets::I686UnknownLinuxMusl => Arch::I686,
            Targets::X8664AppleDarwin => Arch::X8664,
            Targets::X8664UnknownLinuxGnu => Arch::X8664,
            Targets::X8664UnknownLinuxMusl => Arch::X8664,
        }
    }
}

impl From<&mut Targets> for Arch {
    fn from(target: &mut Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => Arch::Aarch64,
            Targets::Aarch64UnknownLinuxGnu => Arch::Aarch64,
            Targets::Aarch64UnknownLinuxMusl => Arch::Aarch64,
            Targets::I686UnknownLinuxGnu => Arch::I686,
            Targets::I686UnknownLinuxMusl => Arch::I686,
            Targets::X8664AppleDarwin => Arch::X8664,
            Targets::X8664UnknownLinuxGnu => Arch::X8664,
            Targets::X8664UnknownLinuxMusl => Arch::X8664,
        }
    }
}

impl Display for Targets {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Targets::Aarch64AppleDarwin => write!(f, "aarch64-apple-darwin"),
            Targets::Aarch64UnknownLinuxGnu => write!(f, "aarch64-unknown-linux-gnu"),
            Targets::Aarch64UnknownLinuxMusl => write!(f, "aarch64-unknown-linux-musl"),
            Targets::I686UnknownLinuxGnu => write!(f, "i686-unknown-linux-gnu"),
            Targets::I686UnknownLinuxMusl => write!(f, "i686-unknown-linux-musl"),
            Targets::X8664AppleDarwin => write!(f, "x86_64-apple-darwin"),
            Targets::X8664UnknownLinuxGnu => write!(f, "x86_64-unknown-linux-gnu"),
            Targets::X8664UnknownLinuxMusl => write!(f, "x86_64-unknown-linux-musl"),
        }
    }
}

impl TryFrom<String> for Targets {
    type Error = XtaskError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "aarch64-apple-darwin" => Ok(Targets::Aarch64AppleDarwin),
            "aarch64-unknown-linux-gnu" => Ok(Targets::Aarch64UnknownLinuxGnu),
            "aarch64-unknown-linux-musl" => Ok(Targets::Aarch64UnknownLinuxMusl),
            "i686-unknown-linux-gnu" => Ok(Targets::I686UnknownLinuxGnu),
            "i686-unknown-linux-musl" => Ok(Targets::I686UnknownLinuxMusl),
            "x86_64-apple-darwin" => Ok(Targets::X8664AppleDarwin),
            "x86_64-unknown-linux-gnu" => Ok(Targets::X8664UnknownLinuxGnu),
            "x86_64-unknown-linux-musl" => Ok(Targets::X8664UnknownLinuxMusl),
            _ => Err(XtaskError::AnyError(format!(
                "Invalid target string {}",
                value
            ))),
        }
    }
}

impl TryFrom<&String> for Targets {
    type Error = XtaskError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "aarch64-apple-darwin" => Ok(Targets::Aarch64AppleDarwin),
            "aarch64-unknown-linux-gnu" => Ok(Targets::Aarch64UnknownLinuxGnu),
            "aarch64-unknown-linux-musl" => Ok(Targets::Aarch64UnknownLinuxMusl),
            "i686-unknown-linux-gnu" => Ok(Targets::I686UnknownLinuxGnu),
            "i686-unknown-linux-musl" => Ok(Targets::I686UnknownLinuxMusl),
            "x86_64-apple-darwin" => Ok(Targets::X8664AppleDarwin),
            "x86_64-unknown-linux-gnu" => Ok(Targets::X8664UnknownLinuxGnu),
            "x86_64-unknown-linux-musl" => Ok(Targets::X8664UnknownLinuxMusl),
            _ => Err(XtaskError::AnyError(format!(
                "Invalid target string {}",
                value
            ))),
        }
    }
}

impl TryFrom<&mut String> for Targets {
    type Error = XtaskError;

    fn try_from(value: &mut String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "aarch64-apple-darwin" => Ok(Targets::Aarch64AppleDarwin),
            "aarch64-unknown-linux-gnu" => Ok(Targets::Aarch64UnknownLinuxGnu),
            "aarch64-unknown-linux-musl" => Ok(Targets::Aarch64UnknownLinuxMusl),
            "i686-unknown-linux-gnu" => Ok(Targets::I686UnknownLinuxGnu),
            "i686-unknown-linux-musl" => Ok(Targets::I686UnknownLinuxMusl),
            "x86_64-apple-darwin" => Ok(Targets::X8664AppleDarwin),
            "x86_64-unknown-linux-gnu" => Ok(Targets::X8664UnknownLinuxGnu),
            "x86_64-unknown-linux-musl" => Ok(Targets::X8664UnknownLinuxMusl),
            _ => Err(XtaskError::AnyError(format!(
                "Invalid target string {}",
                value
            ))),
        }
    }
}

impl TryFrom<&str> for Targets {
    type Error = XtaskError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "aarch64-apple-darwin" => Ok(Targets::Aarch64AppleDarwin),
            "aarch64-unknown-linux-gnu" => Ok(Targets::Aarch64UnknownLinuxGnu),
            "aarch64-unknown-linux-musl" => Ok(Targets::Aarch64UnknownLinuxMusl),
            "i686-unknown-linux-gnu" => Ok(Targets::I686UnknownLinuxGnu),
            "i686-unknown-linux-musl" => Ok(Targets::I686UnknownLinuxMusl),
            "x86_64-apple-darwin" => Ok(Targets::X8664AppleDarwin),
            "x86_64-unknown-linux-gnu" => Ok(Targets::X8664UnknownLinuxGnu),
            "x86_64-unknown-linux-musl" => Ok(Targets::X8664UnknownLinuxMusl),
            _ => Err(XtaskError::AnyError(format!(
                "Invalid target string {}",
                value
            ))),
        }
    }
}

impl TryFrom<&str> for Arch {
    type Error = XtaskError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .as_str()
        {
            "arm" | "aarch64" | "arm64" | "armv8r" | "armv8a" | "armv81a" | "armv82a"
            | "armv83a" | "armv84a" | "armv85a" | "armv86a" | "armv87a" | "armv88a" | "armv89a"
            | "armv90a" | "armv91a" | "armv92a" | "armv93a" | "armv94a" | "armv95a" | "armv96a"
            | "aarch32" | "armv9r" | "armv9m" | "armv8m" | "armv7a" | "armv7r" | "armv7h"
            | "armv7em" | "armv7m" | "armv7l" => Ok(Arch::Aarch64),
            "i686" | "x86" | "amd32" => Ok(Arch::I686),
            "x8664" | "amd64" => Ok(Arch::X8664),
            _ => Err(XtaskError::AnyError(format!(
                "Invalid target string {}",
                value
            ))),
        }
    }
}

impl TryFrom<String> for Arch {
    type Error = XtaskError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .as_str()
        {
            "arm" | "aarch64" | "arm64" | "armv8r" | "armv8a" | "armv81a" | "armv82a"
            | "armv83a" | "armv84a" | "armv85a" | "armv86a" | "armv87a" | "armv88a" | "armv89a"
            | "armv90a" | "armv91a" | "armv92a" | "armv93a" | "armv94a" | "armv95a" | "armv96a"
            | "aarch32" | "armv9r" | "armv9m" | "armv8m" | "armv7a" | "armv7r" | "armv7h"
            | "armv7em" | "armv7m" | "armv7l" => Ok(Arch::Aarch64),
            "i686" | "x86" | "amd32" => Ok(Arch::I686),
            "x8664" | "amd64" => Ok(Arch::X8664),
            _ => Err(XtaskError::AnyError(format!(
                "Invalid target string {}",
                value
            ))),
        }
    }
}

impl TryFrom<&String> for Arch {
    type Error = XtaskError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .as_str()
        {
            "arm" | "aarch64" | "arm64" | "armv8r" | "armv8a" | "armv81a" | "armv82a"
            | "armv83a" | "armv84a" | "armv85a" | "armv86a" | "armv87a" | "armv88a" | "armv89a"
            | "armv90a" | "armv91a" | "armv92a" | "armv93a" | "armv94a" | "armv95a" | "armv96a"
            | "aarch32" | "armv9r" | "armv9m" | "armv8m" | "armv7a" | "armv7r" | "armv7h"
            | "armv7em" | "armv7m" | "armv7l" => Ok(Arch::Aarch64),
            "i686" | "x86" | "amd32" => Ok(Arch::I686),
            "x8664" | "amd64" => Ok(Arch::X8664),
            _ => Err(XtaskError::AnyError(format!(
                "Invalid target string {}",
                value
            ))),
        }
    }
}

impl TryFrom<&mut String> for Arch {
    type Error = XtaskError;

    fn try_from(value: &mut String) -> Result<Self, Self::Error> {
        match value
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .as_str()
        {
            "arm" | "aarch64" | "arm64" | "armv8r" | "armv8a" | "armv81a" | "armv82a"
            | "armv83a" | "armv84a" | "armv85a" | "armv86a" | "armv87a" | "armv88a" | "armv89a"
            | "armv90a" | "armv91a" | "armv92a" | "armv93a" | "armv94a" | "armv95a" | "armv96a"
            | "aarch32" | "armv9r" | "armv9m" | "armv8m" | "armv7a" | "armv7r" | "armv7h"
            | "armv7em" | "armv7m" | "armv7l" => Ok(Arch::Aarch64),
            "i686" | "x86" | "amd32" => Ok(Arch::I686),
            "x8664" | "amd64" => Ok(Arch::X8664),
            _ => Err(XtaskError::AnyError(format!(
                "Invalid target string {}",
                value
            ))),
        }
    }
}

impl TryFrom<Info> for Targets {
    type Error = XtaskError;

    fn try_from(info: Info) -> Result<Self, Self::Error> {
        match info.architecture() {
            Some(arch) => match arch
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .as_str()
            {
                "arm" | "aarch64" | "arm64" | "armv8r" | "armv8a" | "armv81a" | "armv82a"
                | "armv83a" | "armv84a" | "armv85a" | "armv86a" | "armv87a" | "armv88a"
                | "armv89a" | "armv90a" | "armv91a" | "armv92a" | "armv93a" | "armv94a"
                | "armv95a" | "armv96a" | "aarch32" | "armv9r" | "armv9m" | "armv8m" | "armv7a"
                | "armv7r" | "armv7h" | "armv7em" | "armv7m" | "armv7l" => match info.os_type() {
                    Type::Macos => Ok(Targets::Aarch64AppleDarwin),
                    Type::Linux
                    | Type::AlmaLinux
                    | Type::Alpaquita
                    | Type::Alpine
                    | Type::Amazon
                    | Type::Arch
                    | Type::Artix
                    | Type::Bluefin
                    | Type::CachyOS
                    | Type::CentOS
                    | Type::Debian
                    | Type::EndeavourOS
                    | Type::Fedora
                    | Type::Garuda
                    | Type::Gentoo
                    | Type::Kali
                    | Type::Mabox
                    | Type::Manjaro
                    | Type::Mariner
                    | Type::Mint
                    | Type::NixOS
                    | Type::Nobara
                    | Type::OpenCloudOS
                    | Type::openEuler
                    | Type::openSUSE
                    | Type::OracleLinux
                    | Type::Pop
                    | Type::Raspbian
                    | Type::Redhat
                    | Type::RedHatEnterprise
                    | Type::RockyLinux
                    | Type::Solus
                    | Type::SUSE
                    | Type::Ubuntu
                    | Type::Void => match info.bitness() {
                        Bitness::X64 => Ok(Targets::Aarch64UnknownLinuxGnu),
                        _ => Ok(Targets::Aarch64UnknownLinuxMusl),
                    },
                    os_type => Err(XtaskError::AnyError(format!(
                        "Unsupported OS type: {}",
                        os_type
                    ))),
                },
                "x8664" | "amd64" => match info.os_type() {
                    Type::Macos => Ok(Targets::X8664AppleDarwin),
                    Type::Linux
                    | Type::AlmaLinux
                    | Type::Alpaquita
                    | Type::Alpine
                    | Type::Amazon
                    | Type::Arch
                    | Type::Artix
                    | Type::Bluefin
                    | Type::CachyOS
                    | Type::CentOS
                    | Type::Debian
                    | Type::EndeavourOS
                    | Type::Fedora
                    | Type::Garuda
                    | Type::Gentoo
                    | Type::Kali
                    | Type::Mabox
                    | Type::Manjaro
                    | Type::Mariner
                    | Type::Mint
                    | Type::NixOS
                    | Type::Nobara
                    | Type::OpenCloudOS
                    | Type::openEuler
                    | Type::openSUSE
                    | Type::OracleLinux
                    | Type::Pop
                    | Type::Raspbian
                    | Type::Redhat
                    | Type::RedHatEnterprise
                    | Type::RockyLinux
                    | Type::Solus
                    | Type::SUSE
                    | Type::Ubuntu
                    | Type::Void => match info.bitness() {
                        Bitness::X64 => Ok(Targets::X8664UnknownLinuxGnu),
                        _ => Ok(Targets::X8664UnknownLinuxMusl),
                    },
                    os_type => Err(XtaskError::AnyError(format!(
                        "Unsupported OS type: {}",
                        os_type
                    ))),
                },
                "i686" | "x86" | "amd32" => match info.os_type() {
                    Type::Linux
                    | Type::AlmaLinux
                    | Type::Alpaquita
                    | Type::Alpine
                    | Type::Amazon
                    | Type::Arch
                    | Type::Artix
                    | Type::Bluefin
                    | Type::CachyOS
                    | Type::CentOS
                    | Type::Debian
                    | Type::EndeavourOS
                    | Type::Fedora
                    | Type::Garuda
                    | Type::Gentoo
                    | Type::Kali
                    | Type::Mabox
                    | Type::Manjaro
                    | Type::Mariner
                    | Type::Mint
                    | Type::NixOS
                    | Type::Nobara
                    | Type::OpenCloudOS
                    | Type::openEuler
                    | Type::openSUSE
                    | Type::OracleLinux
                    | Type::Pop
                    | Type::Raspbian
                    | Type::Redhat
                    | Type::RedHatEnterprise
                    | Type::RockyLinux
                    | Type::Solus
                    | Type::SUSE
                    | Type::Ubuntu
                    | Type::Void => match info.bitness() {
                        Bitness::X32 => Ok(Targets::I686UnknownLinuxGnu),
                        _ => Ok(Targets::X8664UnknownLinuxMusl),
                    },
                    os_type => Err(XtaskError::AnyError(format!(
                        "Unsupported OS type: {}",
                        os_type
                    ))),
                },
                arch => Err(XtaskError::AnyError(format!(
                    "Unsupported architecture: {}",
                    arch
                ))),
            },
            None => Err(XtaskError::AnyError("Unknown architecture".to_string())),
        }
    }
}

impl From<Targets> for Bitness {
    fn from(target: Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => Bitness::X64,
            Targets::Aarch64UnknownLinuxGnu => Bitness::X64,
            Targets::Aarch64UnknownLinuxMusl => Bitness::X64,
            Targets::I686UnknownLinuxGnu => Bitness::X32,
            Targets::I686UnknownLinuxMusl => Bitness::X32,
            Targets::X8664AppleDarwin => Bitness::X64,
            Targets::X8664UnknownLinuxGnu => Bitness::X64,
            Targets::X8664UnknownLinuxMusl => Bitness::X64,
        }
    }
}

impl From<&Targets> for Bitness {
    fn from(target: &Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => Bitness::X64,
            Targets::Aarch64UnknownLinuxGnu => Bitness::X64,
            Targets::Aarch64UnknownLinuxMusl => Bitness::X64,
            Targets::I686UnknownLinuxGnu => Bitness::X32,
            Targets::I686UnknownLinuxMusl => Bitness::X32,
            Targets::X8664AppleDarwin => Bitness::X64,
            Targets::X8664UnknownLinuxGnu => Bitness::X64,
            Targets::X8664UnknownLinuxMusl => Bitness::X64,
        }
    }
}

impl From<&mut Targets> for Bitness {
    fn from(target: &mut Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => Bitness::X64,
            Targets::Aarch64UnknownLinuxGnu => Bitness::X64,
            Targets::Aarch64UnknownLinuxMusl => Bitness::X64,
            Targets::I686UnknownLinuxGnu => Bitness::X32,
            Targets::I686UnknownLinuxMusl => Bitness::X32,
            Targets::X8664AppleDarwin => Bitness::X64,
            Targets::X8664UnknownLinuxGnu => Bitness::X64,
            Targets::X8664UnknownLinuxMusl => Bitness::X64,
        }
    }
}

impl From<Targets> for Type {
    fn from(target: Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => Type::Macos,
            Targets::Aarch64UnknownLinuxGnu => Type::Linux,
            Targets::Aarch64UnknownLinuxMusl => Type::Linux,
            Targets::I686UnknownLinuxGnu => Type::Linux,
            Targets::I686UnknownLinuxMusl => Type::Linux,
            Targets::X8664AppleDarwin => Type::Macos,
            Targets::X8664UnknownLinuxGnu => Type::Linux,
            Targets::X8664UnknownLinuxMusl => Type::Linux,
        }
    }
}

impl From<&Targets> for Type {
    fn from(target: &Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => Type::Macos,
            Targets::Aarch64UnknownLinuxGnu => Type::Linux,
            Targets::Aarch64UnknownLinuxMusl => Type::Linux,
            Targets::I686UnknownLinuxGnu => Type::Linux,
            Targets::I686UnknownLinuxMusl => Type::Linux,
            Targets::X8664AppleDarwin => Type::Macos,
            Targets::X8664UnknownLinuxGnu => Type::Linux,
            Targets::X8664UnknownLinuxMusl => Type::Linux,
        }
    }
}

impl From<&mut Targets> for Type {
    fn from(target: &mut Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => Type::Macos,
            Targets::Aarch64UnknownLinuxGnu => Type::Linux,
            Targets::Aarch64UnknownLinuxMusl => Type::Linux,
            Targets::I686UnknownLinuxGnu => Type::Linux,
            Targets::I686UnknownLinuxMusl => Type::Linux,
            Targets::X8664AppleDarwin => Type::Macos,
            Targets::X8664UnknownLinuxGnu => Type::Linux,
            Targets::X8664UnknownLinuxMusl => Type::Linux,
        }
    }
}

impl From<Targets> for &'static str {
    fn from(target: Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => "aarch64-apple-darwin",
            Targets::Aarch64UnknownLinuxGnu => "aarch64-unknown-linux-gnu",
            Targets::Aarch64UnknownLinuxMusl => "aarch64-unknown-linux-musl",
            Targets::I686UnknownLinuxGnu => "i686-unknown-linux-gnu",
            Targets::I686UnknownLinuxMusl => "i686-unknown-linux-musl",
            Targets::X8664AppleDarwin => "x86_64-apple-darwin",
            Targets::X8664UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
            Targets::X8664UnknownLinuxMusl => "x86_64-unknown-linux-musl",
        }
    }
}

impl From<&Targets> for &'static str {
    fn from(target: &Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => "aarch64-apple-darwin",
            Targets::Aarch64UnknownLinuxGnu => "aarch64-unknown-linux-gnu",
            Targets::Aarch64UnknownLinuxMusl => "aarch64-unknown-linux-musl",
            Targets::I686UnknownLinuxGnu => "i686-unknown-linux-gnu",
            Targets::I686UnknownLinuxMusl => "i686-unknown-linux-musl",
            Targets::X8664AppleDarwin => "x86_64-apple-darwin",
            Targets::X8664UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
            Targets::X8664UnknownLinuxMusl => "x86_64-unknown-linux-musl",
        }
    }
}

impl From<&mut Targets> for &'static str {
    fn from(target: &mut Targets) -> Self {
        match target {
            Targets::Aarch64AppleDarwin => "aarch64-apple-darwin",
            Targets::Aarch64UnknownLinuxGnu => "aarch64-unknown-linux-gnu",
            Targets::Aarch64UnknownLinuxMusl => "aarch64-unknown-linux-musl",
            Targets::I686UnknownLinuxGnu => "i686-unknown-linux-gnu",
            Targets::I686UnknownLinuxMusl => "i686-unknown-linux-musl",
            Targets::X8664AppleDarwin => "x86_64-apple-darwin",
            Targets::X8664UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
            Targets::X8664UnknownLinuxMusl => "x86_64-unknown-linux-musl",
        }
    }
}
