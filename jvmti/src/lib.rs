// TODO #![no_std]?
#![allow(dead_code)]

#[macro_use]
mod util;

mod env;
mod event;

pub use env::JvmtiEnv;
pub use event::{EventCallbacks, EventCallbacksBuilder, EventScope, EventType};
pub use util::{Error, JvmtiResult};
