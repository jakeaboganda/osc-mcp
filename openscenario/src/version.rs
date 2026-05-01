use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpenScenarioVersion {
    V1_0,
    V1_1,
    V1_2,
}

impl OpenScenarioVersion {
    /// Parse version from revMajor and revMinor attributes
    pub fn from_rev(major: u8, minor: u8) -> Option<Self> {
        match (major, minor) {
            (1, 0) => Some(Self::V1_0),
            (1, 1) => Some(Self::V1_1),
            (1, 2) => Some(Self::V1_2),
            _ => None,
        }
    }

    pub fn major(&self) -> u8 {
        1
    }

    pub fn minor(&self) -> u8 {
        match self {
            Self::V1_0 => 0,
            Self::V1_1 => 1,
            Self::V1_2 => 2,
        }
    }
}

impl fmt::Display for OpenScenarioVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major(), self.minor())
    }
}
