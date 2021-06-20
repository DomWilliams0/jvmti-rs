use std::os::raw::c_void;
use std::ptr::null_mut;

use jni::errors::{jni_error_code_to_result, Error, JniError};
use jni::sys::{jint, JNI_OK};
use jni::JavaVM;

use jni_jvmti_sys::{jvmtiEnv, JVMTI_VERSION_1_1};

use crate::util::*;

pub struct JvmtiEnv(*mut jvmtiEnv);

impl JvmtiEnv {
    pub fn from_jvm(jvm: &JavaVM) -> JvmtiResult<Self> {
        let jvm_ptr = jvm.get_java_vm_pointer();
        let mut jvmti_ptr: *mut c_void = null_mut();

        unsafe {
            if jvm_ptr.is_null() || (*jvm_ptr).is_null() {
                return Err(JvmtiError::Jni(GeneralJniError::NullPtr("JavaVM")));
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
            assert!(!jvmti_ptr.is_null());
        }

        debug!("got jvmtiEnv ptr {:?}", jvmti_ptr);
        Ok(Self(jvmti_ptr as *mut jvmtiEnv))
    }
}

#[cfg(test)]
mod tests {
    use jni::{InitArgs, InitArgsBuilder, JNIVersion};

    use super::*;

    #[test]
    fn get_env() {
        env_logger::builder()
            .filter_level(LevelFilter::Trace)
            .is_test(true)
            .try_init();

        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .build()
            .unwrap();

        let jvm = JavaVM::new(jvm_args).unwrap();
        jvm.attach_current_thread_permanently().unwrap();

        let jvmti = JvmtiEnv::from_jvm(&jvm).expect("failed");
    }
}
