pub use crate::error::Error;
pub use std::format as f;

pub type R<T> = core::result::Result<T, Error>;

pub struct W<T>(pub T);
