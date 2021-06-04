use thiserror::Error as ErrorDerive;

pub use super::lexer::LexError;
pub use super::render::RenderError;
pub use super::tags::TagError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ErrorDerive, Debug, PartialEq)]
pub enum Error {
    #[error("{0}")]
    LexError(#[from] LexError),
    #[error("{0}")]
    RenderError(#[from] RenderError),
    #[error("{0}")]
    TagError(#[from] TagError),
}
