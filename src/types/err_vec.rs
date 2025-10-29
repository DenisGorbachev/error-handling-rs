use std::error::Error;
use thiserror::Error;

#[derive(Error, Default, Debug)]
#[error("encountered {len} errors", len = self.inner.len())]
pub struct ErrVec {
    pub inner: Vec<Box<dyn Error + 'static>>,
}
