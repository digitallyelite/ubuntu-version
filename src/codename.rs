use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Error)]
pub enum CodenameParseError {
    #[error("unknown codename string")]
    NotFound,
}

/// The codename associated with an Ubuntu version.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Codename {
    Bionic,
    Cosmic,
    Disco,
    Eoan,
    Focal,
}

impl Codename {
    /// Returns the release date in a `(year, month, date)` format
    pub fn release_date(self) -> (u32, u32, u32) {
        match self {
            Codename::Bionic => (2018, 4, 26),
            Codename::Cosmic => (2018, 10, 18),
            Codename::Disco => (2019, 4, 18),
            Codename::Eoan => (2019, 10, 17),
            // Approximate time for future release
            Codename::Focal => (2020, 4, 0),
        }
    }

    /// When this was released, as the time in seconds since the Unix Epoch
    pub fn release_timestamp(self) -> u64 {
        match self {
            Codename::Bionic => 1_524_700_800,
            Codename::Cosmic => 1_539_820_800,
            Codename::Disco => 1_555_545_600,
            Codename::Eoan => 1_571_270_400,
            // Approximate time for future release
            Codename::Focal => 1_585_699_200,
        }
    }
}

impl Display for Codename {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { fmt.write_str(<&'static str>::from(*self)) }
}

impl FromStr for Codename {
    type Err = CodenameParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let release = match input {
            "bionic" => Codename::Bionic,
            "cosmic" => Codename::Cosmic,
            "disco" => Codename::Disco,
            "eoan" => Codename::Eoan,
            "focal" => Codename::Focal,
            _ => return Err(CodenameParseError::NotFound),
        };

        Ok(release)
    }
}

impl From<Codename> for &'static str {
    fn from(codename: Codename) -> Self {
        match codename {
            Codename::Bionic => "bionic",
            Codename::Cosmic => "cosmic",
            Codename::Disco => "disco",
            Codename::Eoan => "eoan",
            Codename::Focal => "focal",
        }
    }
}
