// TODO #![no_std]?
#![allow(dead_code)]

#[macro_use]
mod util;

mod capability;
mod env;
mod event;
mod memory;

pub use env::JvmtiEnv;
pub use event::{EventCallbacks, EventCallbacksBuilder, EventScope, EventType};
pub use util::{Error, JvmtiResult};
