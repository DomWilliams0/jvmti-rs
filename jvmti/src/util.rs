pub use displaydoc::Display;
pub use jni::errors::Error as GeneralJniError;
pub use log::*;
pub use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum JvmtiError {
    /// JNI error: {0}
    Jni(#[from] GeneralJniError),
}

pub type JvmtiResult<T> = Result<T, JvmtiError>;
