#[macro_use]
extern crate thiserror;

mod codename;
mod version;

pub use self::{codename::*, version::*};

impl From<Version> for Codename {
    fn from(version: Version) -> Codename {
        match (version.major, version.minor) {
            (18, 4) => Codename::Bionic,
            (18, 10) => Codename::Cosmic,
            (19, 4) => Codename::Disco,
            (19, 10) => Codename::Eoan,
            (20, 4) => Codename::Focal,
            _ => panic!("unsupported ubuntu version"),
        }
    }
}

impl From<Codename> for Version {
    fn from(codename: Codename) -> Version {
        let (major, minor) = match codename {
            Codename::Bionic => (18, 4),
            Codename::Cosmic => (18, 10),
            Codename::Disco => (19, 4),
            Codename::Eoan => (19, 10),
            Codename::Focal => (20, 4),
        };

        Version { major, minor, patch: 0 }
    }
}
