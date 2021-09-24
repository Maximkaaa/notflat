#[cfg(not(feature = "no-std"))]
use thiserror::Error;

#[cfg(not(feature = "no-std"))]
# [derive(Error, Debug)]
pub enum NotFlatError {
    # [error("invalid latitude value: {0}")]
    InvalidLatitude(f64),
}

#[cfg(feature = "no-std")]
#[derive(Debug)]
pub enum NotFlatError {
    InvalidLatitude(f64),
}