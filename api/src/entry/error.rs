use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    Custom(&'static str),
    NotImplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(msg) => f.write_str(msg),
            Self::NotImplemented => f.write_str("Rendering this variant is not implemented"),
        }
    }
}
