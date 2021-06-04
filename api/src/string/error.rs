use thiserror::Error as ErrorDerive;

pub use super::lexer::LexError;
pub use super::render::RenderError;
pub use super::tags::TagError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ErrorDerive, Debug, PartialEq)]
pub enum Error {
    #[error("Error while tokenizing: {0}")]
    LexError(#[from] LexError),
    #[error("Error while rendering: {0}")]
    RenderError(#[from] RenderError),
    #[error("Error with a tag: {0}")]
    TagError(#[from] TagError),
}
