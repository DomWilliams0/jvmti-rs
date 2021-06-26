use jni_jvmti_sys::jvmtiCapabilities;
use jvmti::{HeapFilterFlags, HeapVisitControlFlags, JvmtiEnv};
use log::*;

mod common;

#[test]
fn iterate_through_heap() {
    let jvm = common::new_jvm();
    let _jni = jvm.attach_current_thread().unwrap();

    let jvmti = JvmtiEnv::from_jvm(&jvm).expect("failed");

    {
        let mut capabilities = jvmtiCapabilities::default();
        capabilities.set_can_tag_objects(1);
        jvmti
            .add_capabilities(&capabilities)
            .expect("failed to add capabilities");
    }

    jvmti
        .iterate_through_heap(HeapFilterFlags::empty(), None, |obj| {
            // keep visiting and dont recurse fields
            debug!("obj {:?}", obj);

            HeapVisitControlFlags::VISIT_OBJECTS
        })
        .expect("iteration failed");

    jvmti.dispose().expect("dispose failed");
}
