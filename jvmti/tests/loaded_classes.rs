use jni::objects::{JClass, JValue};
use jni::signature::{JavaType, Primitive};
use jvmti::JvmtiEnv;

mod common;

#[test]
fn loaded_classes() {
    let jvm = common::new_jvm();
    let jni = jvm.attach_current_thread().unwrap();

    let jvmti = JvmtiEnv::from_jvm(&jvm).expect("failed");

    // cause a class to be loaded
    let math_cls = jni
        .find_class("java/lang/Math")
        .expect("failed to find math")
        .into_inner();

    let cls_eq_method = jni
        .get_method_id("java/lang/Class", "equals", "(Ljava/lang/Object;)Z")
        .expect("cant find equals()");

    let mut math_cls_found = false;
    {
        let loaded_classes = jvmti.get_loaded_classes(*jni).expect("failed");
        for cls in &*loaded_classes {
            let cls = JClass::from(*cls);
            if jni
                .call_method_unchecked(
                    math_cls,
                    cls_eq_method,
                    JavaType::Primitive(Primitive::Boolean),
                    &[JValue::Object(cls.into())],
                )
                .expect("eq failed")
                .z()
                .expect("not a bool")
            {
                math_cls_found = true;
            }
        }
    }

    assert!(math_cls_found);

    jvmti.dispose().expect("dispose failed");
}
