use crate::util::*;
use crate::JvmtiEnv;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;

use mutf8::mstr;
use std::ffi::CStr;

use std::ops::Deref;
use std::os::raw::c_char;

pub trait Allocation: Sized {
    const WHAT: &'static str;
    type Element;
    fn release_multiple(jni: JNIEnv, array: &[Self::Element]);
}

pub struct AllocatedArray<'a, T: Allocation> {
    array: &'a mut [T::Element],
    jni: JNIEnv<'a>,
    jvmti: JvmtiEnv<'a>,
}

pub struct AllocatedMutf8<'a> {
    str: &'a mutf8::mstr,
    jvmti: JvmtiEnv<'a>,
}

/// jobject
pub struct LocalRef;

impl<'a, T: Allocation> AllocatedArray<'a, T> {
    pub unsafe fn new(
        ptr: *mut T::Element,
        len: usize,
        jni: JNIEnv<'a>,
        jvmti: JvmtiEnv<'a>,
    ) -> Self {
        let slice = std::slice::from_raw_parts_mut(ptr, len);
        Self {
            array: slice,
            jni,
            jvmti,
        }
    }
}

impl<'a> AllocatedMutf8<'a> {
    pub unsafe fn new(nul_terminated_ptr: *mut c_char, jvmti: JvmtiEnv<'a>) -> Self {
        let cstr = CStr::from_ptr(nul_terminated_ptr);
        let str = mutf8::mstr::from_mutf8(cstr.to_bytes());
        Self { str, jvmti }
    }
}

impl<'a, T: Allocation> Drop for AllocatedArray<'a, T> {
    fn drop(&mut self) {
        // free elements
        T::release_multiple(self.jni, self.array);

        // free allocation
        unsafe {
            if let Err(err) = self.jvmti.deallocate(self.array.as_mut_ptr() as *mut ()) {
                error!(
                    "failed to deallocate array of {} {:?}: {}",
                    self.array.len(),
                    T::WHAT,
                    err
                )
            }
        }
    }
}

impl Drop for AllocatedMutf8<'_> {
    fn drop(&mut self) {
        // free allocation
        unsafe {
            if let Err(err) = self.jvmti.deallocate(self.str.as_ptr() as *mut ()) {
                error!(
                    "failed to deallocate mutf8 string of len {}: {}",
                    self.str.len(),
                    err
                )
            }
        }
    }
}

impl<'a, T: Allocation> Deref for AllocatedArray<'a, T> {
    type Target = [T::Element];

    fn deref(&self) -> &Self::Target {
        self.array
    }
}

impl Deref for AllocatedMutf8<'_> {
    type Target = mstr;

    fn deref(&self) -> &Self::Target {
        self.str
    }
}

impl Allocation for LocalRef {
    const WHAT: &'static str = "local refs";
    type Element = jobject;

    fn release_multiple(jni: JNIEnv, array: &[Self::Element]) {
        trace!("releasing {} local refs", array.len());
        for obj in array {
            if let Err(err) = jni.delete_local_ref(JObject::from(*obj)) {
                error!("failed to delete local ref: {}", err);
            }
        }
    }
}
