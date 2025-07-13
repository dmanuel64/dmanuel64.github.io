use std::path::Path;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContentError {
    #[error("Cannot locate '{0}'")]
    ContentNotFound(String),
    #[error("Encountered a UTF8 error while retrieving content")]
    UTF8Error,
}
