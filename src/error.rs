use std::result;

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("RuTel error")]
    RuTel(#[from] rutel::error::Error),
    // #[error("json error")]
    // JSON(#[from] serde_json::Error),
}
