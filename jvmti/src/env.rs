use core::ptr::null_mut;

use jni::errors::jni_error_code_to_result;
use jni::sys::{jclass, jint};
use jni::JavaVM;

use jni_jvmti_sys::{jvmtiEnv, jvmtiEventCallbacks, jvmtiInterface_1_, JVMTI_VERSION_1_1};

use crate::event::{EventCallbacks, EventScope, EventType};
use crate::memory::{AllocatedArray, LocalRef};
use crate::util::*;
use core::ffi::c_void;
use jni_jvmti_sys::jvmtiEventMode::{JVMTI_DISABLE, JVMTI_ENABLE};
use std::marker::PhantomData;

/// Shared across threads.
/// TODO how to dispose via RAII?
#[derive(Clone)]
#[repr(transparent)]
pub struct JvmtiEnv<'a>(*mut jvmtiEnv, PhantomData<&'a ()>);

impl<'a> JvmtiEnv<'a> {
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
        Ok(Self(jvmti_ptr as *mut jvmtiEnv, PhantomData))
    }

    /// Note events must be enabled to be fired
    pub fn install_event_callbacks(&self, callbacks: &EventCallbacks) -> JvmtiResult<()> {
        jvmti_method!(
            self,
            SetEventCallbacks,
            callbacks.into(),
            core::mem::size_of::<jvmtiEventCallbacks>() as jint
        );

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
        jvmti_method!(
            self,
            SetEventNotificationMode,
            if enabled { JVMTI_ENABLE } else { JVMTI_DISABLE },
            ty.into(),
            match scope {
                EventScope::Global => null_mut(),
                EventScope::Thread(thread) => thread,
            }
        );
        debug!(
            "{}abled event type {:?} in scope {:?}",
            if enabled { "en" } else { "dis" }, // lol
            ty,
            scope,
        );
        Ok(())
    }

    // TODO fix lifetimes?
    pub fn get_loaded_classes<'b>(
        &'b self,
        jni: jni::JNIEnv<'b>,
    ) -> JvmtiResult<AllocatedArray<'b, LocalRef>> {
        let mut count: jint = 0;
        let mut classes: *mut jclass = null_mut();
        jvmti_method!(
            self,
            GetLoadedClasses,
            &mut count as *mut jint,
            &mut classes as *mut *mut jclass
        );
        debug!("got {} loaded classes", count);

        Ok(unsafe { AllocatedArray::<LocalRef>::new(classes, count as usize, jni, self.clone()) })
    }

    pub fn dispose(self) -> JvmtiResult<()> {
        jvmti_method!(self, DisposeEnvironment);
        debug!("disposed jvmti environment at {:?}", self.0);
        Ok(())
    }

    /// # Safety
    /// Pointer must be a JVMTI allocation
    pub unsafe fn deallocate(&self, ptr: *mut ()) -> JvmtiResult<()> {
        debug!("deallocating {:?}", ptr);
        jvmti_method!(self, Deallocate, ptr as *mut _);
        Ok(())
    }

    pub(crate) fn as_ref(&self) -> &jvmtiInterface_1_ {
        debug_assert!(!self.0.is_null());
        unsafe { &**self.0 }
    }

    pub(crate) fn as_ptr(&self) -> *mut jvmtiEnv {
        debug_assert!(!self.0.is_null());
        self.0
    }
}
