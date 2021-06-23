use jni::sys::jclass;
use jni_jvmti_sys::jthread;
use jvmti::{EventCallbacksBuilder, EventScope, EventType, JvmtiEnv};

mod common;

#[test]
fn event_callback() {
    let jvm = common::new_jvm();
    let attach = jvm.attach_current_thread().unwrap();

    let jvmti = JvmtiEnv::from_jvm(&jvm).expect("failed");
    jvmti
        .enable_event(EventType::ClassLoad, EventScope::Global)
        .expect("failed");

    static mut CALLBACK_HIT: bool = false;
    unsafe extern "C" fn classload_callback(
        _jvmti_env: JvmtiEnv,
        _jni_env: jni::JNIEnv,
        _thread: jthread,
        _klass: jclass,
    ) {
        log::info!("class load!!");
        CALLBACK_HIT = true;
    }

    let events = EventCallbacksBuilder::default()
        .with_class_load(Some(classload_callback))
        .build();

    jvmti.install_event_callbacks(&events).expect("failed");

    // do something noddy to load a class
    let x = jni::objects::JValue::from(-10);
    attach
        .call_static_method("java/lang/Math", "abs", "(I)I", &[x])
        .expect("failed");

    assert!(
        unsafe { CALLBACK_HIT },
        "class load event callback not called"
    );

    jvmti.dispose().expect("dispose failed");
}
