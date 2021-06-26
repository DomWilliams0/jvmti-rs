use jni::objects::JString;
use jni_jvmti_sys::jvmtiCapabilities;
use jvmti::{HeapFilterFlags, HeapIterationCallback, HeapVisitControlFlags, JvmtiEnv};
use log::*;

mod common;

#[test]
fn iterate_through_heap_and_tag() {
    let jvm = common::new_jvm();
    let jni = jvm.attach_current_thread().unwrap();

    let jvmti = JvmtiEnv::from_jvm(&jvm).expect("failed");

    {
        let mut capabilities = jvmtiCapabilities::default();
        capabilities.set_can_tag_objects(1);
        jvmti
            .add_capabilities(&capabilities)
            .expect("failed to add capabilities");
    }

    let string_tag = 0x91828312712;
    let mut prints_left = 500; // dont spam too much
    jvmti
        .iterate_through_heap(HeapFilterFlags::empty(), None, |obj| {
            if prints_left >= 0 {
                debug!("{:?}", obj);
                prints_left -= 1;
            }

            // tag strings
            if let HeapIterationCallback::String { tag, .. } = obj {
                *tag = string_tag;
            }

            // visit everything
            HeapVisitControlFlags::VISIT_OBJECTS
        })
        .expect("iteration failed");

    {
        let tagged_objs = jvmti
            .get_objects_with_tag(string_tag, *jni)
            .expect("failed");
        for string in &*tagged_objs {
            let chars = jni.get_string(JString::from(*string)).expect("bad string");
            debug!("string {:?}", chars.to_string_lossy());
        }
    }

    jvmti.dispose().expect("dispose failed");
}

// TODO do the same but with followreferences
