pub use displaydoc::Display;
pub use jni::errors::Error as GeneralJniError;

use jni_jvmti_sys::jvmtiError;

pub use log::*;
pub use thiserror::Error;

#[derive(Debug, Error, Display)]
pub enum Error {
    /// JNI error: {0}
    Jni(#[from] GeneralJniError),

    /// JVMTI error: {0}
    Jvmti(#[from] JvmtiError),

    /// JVMTI function {0:?} is null
    NullFunction(&'static str),
}

/// Maps to JVMTI_ERROR_*
#[derive(Debug, Error, Display)]
pub enum JvmtiError {
    // TODO actually add variants for errors
    /// The capability being used is false in this environment.
    MissingCapability,

    /// {0:?}
    Other(jvmtiError),
}

pub type JvmtiResult<T> = Result<T, Error>;

pub fn jvmti_err_to_result(err: jvmtiError) -> Result<(), JvmtiError> {
    use jvmtiError::*;
    use JvmtiError::*;
    Err(match err {
        JVMTI_ERROR_NONE => return Ok(()),
        JVMTI_ERROR_MUST_POSSESS_CAPABILITY => MissingCapability,
        err => Other(err),
    })
}
