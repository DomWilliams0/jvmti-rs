use jni_jvmti_sys::jvmtiCapabilities;
use jvmti::{EventScope, EventType, JvmtiEnv};

mod common;

#[test]
fn capabilities() {
    let jvm = common::new_jvm();
    let _env = jvm.attach_current_thread().unwrap();

    let jvmti = JvmtiEnv::from_jvm(&jvm).expect("failed");
    jvmti
        .enable_event(EventType::ClassLoad, EventScope::Global)
        .expect("failed");

    let potential = jvmti.get_potential_capabilities().expect("failed");
    let active = jvmti.get_capabilities().expect("failed");

    assert_eq!(potential.can_tag_objects(), 1);
    assert_eq!(active.can_tag_objects(), 0);

    let mut new = jvmtiCapabilities::default();
    new.set_can_tag_objects(1);
    jvmti.add_capabilities(&new).expect("failed");

    let active = jvmti.get_capabilities().expect("failed");
    assert_eq!(active.can_tag_objects(), 1);

    jvmti.relinquish_capabilities(&new).expect("failed");
    let active = jvmti.get_capabilities().expect("failed");
    assert_eq!(active.can_tag_objects(), 0);

    jvmti.dispose().expect("dispose failed");
}
