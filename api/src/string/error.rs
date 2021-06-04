use super::tags::TagName;
use std::fmt;
use std::str::FromStr;

pub use super::lexer::LexErrorKind;
pub use super::render::RenderErrorKind;
pub use super::tags::TagErrorKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Errors that occur while lexing (tokenizing) the input string.
    LexError(LexErrorKind),
    /// Errors that occur during rendering.
    RenderError(RenderErrorKind),
    /// Errors that occur while parsing tags.
    TagError(TagErrorKind),
    /// Any other error.
    Custom(String),
}

impl From<LexErrorKind> for Error {
    fn from(value: LexErrorKind) -> Self {
        Self::LexError(value)
    }
}

impl From<RenderErrorKind> for Error {
    fn from(value: RenderErrorKind) -> Self {
        Self::RenderError(value)
    }
}

impl From<<TagName as FromStr>::Err> for Error {
    fn from(value: <TagName as FromStr>::Err) -> Self {
        Self::TagError(value)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(message: &'a str) -> Self {
        Self::Custom(message.to_owned())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LexError(kind) => write!(f, "A lexical error occurred: {:?}", kind),
            Self::RenderError(kind) => write!(f, "A rendering error occurred: {:?}", kind),
            Self::TagError(kind) => write!(f, "An error occurred with a tag: {:?}", kind),
            Self::Custom(message) => write!(f, "{}", message),
        }
    }
}
