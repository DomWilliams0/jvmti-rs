use core::ptr::null_mut;

use jni::errors::jni_error_code_to_result;
use jni::sys::jint;
use jni::JavaVM;

use jni_jvmti_sys::{jvmtiEnv, jvmtiEventCallbacks, jvmtiInterface_1_, JVMTI_VERSION_1_1};

use crate::event::{EventCallbacks, EventScope, EventType};
use crate::util::*;
use core::ffi::c_void;
use jni_jvmti_sys::jvmtiEventMode::{JVMTI_DISABLE, JVMTI_ENABLE};

#[repr(transparent)]
pub struct JvmtiEnv(*mut jvmtiEnv);

impl JvmtiEnv {
    pub fn from_jvm(jvm: &JavaVM) -> JvmtiResult<Self> {
        let jvm_ptr = jvm.get_java_vm_pointer();
        let mut jvmti_ptr: *mut c_void = null_mut();

        unsafe {
            if jvm_ptr.is_null() || (*jvm_ptr).is_null() {
                return Err(GeneralJniError::NullPtr("JavaVM").into());
            }

            let get_env_fn = (**jvm_ptr)
                .GetEnv
                .ok_or(GeneralJniError::NullPtr("GetEnv"))?;

            let ret = get_env_fn(
                jvm_ptr,
                (&mut jvmti_ptr) as *mut *mut c_void,
                JVMTI_VERSION_1_1 as jint,
            );
            jni_error_code_to_result(ret)?;
        }

        assert!(!jvmti_ptr.is_null());
        debug!("got jvmtiEnv ptr {:?}", jvmti_ptr);
        Ok(Self(jvmti_ptr as *mut jvmtiEnv))
    }

    /// Note events must be enabled to be fired
    pub fn install_event_callbacks(&self, callbacks: &EventCallbacks) -> JvmtiResult<()> {
        // TODO macro for function calling
        let fn_ptr = self
            .as_ref()
            .SetEventCallbacks
            .ok_or(Error::NullFunction("SetEventCallbacks"))?;

        let ret = unsafe {
            fn_ptr(
                self.0,
                callbacks.into(),
                core::mem::size_of::<jvmtiEventCallbacks>() as jint,
            )
        };

        jvmti_err_to_result(ret)?;

        debug!("installed event callbacks: {:?}", callbacks);
        Ok(())
    }

    pub fn enable_event(&self, ty: EventType, scope: EventScope) -> JvmtiResult<()> {
        self.set_event_enabled(ty, scope, true)
    }

    pub fn disable_event(&self, ty: EventType, scope: EventScope) -> JvmtiResult<()> {
        self.set_event_enabled(ty, scope, false)
    }

    pub fn set_event_enabled(
        &self,
        ty: EventType,
        scope: EventScope,
        enabled: bool,
    ) -> JvmtiResult<()> {
        // TODO macro for function calling
        let fn_ptr = self
            .as_ref()
            .SetEventNotificationMode
            .ok_or(Error::NullFunction("SetEventNotificationMode"))?;

        let ret = unsafe {
            fn_ptr(
                self.0,
                if enabled { JVMTI_ENABLE } else { JVMTI_DISABLE },
                ty.into(),
                match scope {
                    EventScope::Global => null_mut(),
                    EventScope::Thread(thread) => thread,
                },
            )
        };

        jvmti_err_to_result(ret)?;
        debug!(
            "{}abled event type {:?} in scope {:?}",
            if enabled { "en" } else { "dis" }, // lol
            ty,
            scope,
        );
        Ok(())
    }

    fn as_ref(&self) -> &jvmtiInterface_1_ {
        debug_assert!(!self.0.is_null());
        unsafe { &**self.0 }
    }
}
