use std::fmt::{self, Formatter};

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Http);

impl Version {
    pub const HTTP_09: Version = Version(Http::Http09);
    pub const HTTP_10: Version = Version(Http::Http10);
    pub const HTTP_11: Version = Version(Http::Http11);
    pub const HTTP_2: Version = Version(Http::H2);
    pub const HTTP_3: Version = Version(Http::H3);
    pub const UNKNOWN: Version = Version(Http::__NonExhaustive);

    pub fn from_str(src: &str) -> Version {
        match src {
            "HTTP/0.9" => Version::HTTP_09,
            "HTTP/1.0" => Version::HTTP_10,
            "HTTP/1.1" => Version::HTTP_11,
            "HTTP/2" => Version::HTTP_2,
            "HTTP/3" => Version::HTTP_3,
            _ => Version::UNKNOWN,
        }
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
enum Http {
    Http09,
    Http10,
    Http11,
    H2,
    H3,
    __NonExhaustive,
}

impl Default for Version {
    fn default() -> Self {
        Version(Http::Http11)
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use self::Http::*;

        f.write_str(match self.0 {
            Http09 => "HTTP/0.9",
            Http10 => "HTTP/1.0",
            Http11 => "HTTP/1.1",
            H2 => "HTTP/2",
            H3 => "HTTP/3",
            __NonExhaustive => "Unknown",
        })
    }
}
