use super::render::RenderError;
use serde_json::Error as SerdeError;
use thiserror::Error as ErrorDerive;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ErrorDerive, Debug)]
pub enum Error {
    #[error("{0}")]
    RenderError(#[from] RenderError),
    #[error("{0}")]
    SerdeError(#[from] SerdeError),
}
