use crate::util::*;
use crate::JvmtiEnv;
use jni::objects::JObject;
use jni::sys::jobject;
use jni::JNIEnv;

use std::ops::Deref;

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

impl<'a, T: Allocation> Deref for AllocatedArray<'a, T> {
    type Target = [T::Element];

    fn deref(&self) -> &Self::Target {
        self.array
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
