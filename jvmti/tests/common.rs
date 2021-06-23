use jni::{InitArgsBuilder, JNIVersion, JavaVM};
use log::LevelFilter;

/// Current thread is unattached
pub fn new_jvm() -> JavaVM {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .is_test(true)
        .try_init();

    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .build()
        .expect("failed to create jvm args");

    JavaVM::new(jvm_args).expect("failed to create jvm")
}
