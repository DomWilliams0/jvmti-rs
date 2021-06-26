use core::ptr::null_mut;

use jni::errors::jni_error_code_to_result;
use jni::sys::{jchar, jclass, jint, jlong, jobject, jvalue};
use jni::JavaVM;

use crate::event::{EventCallbacks, EventScope, EventType};
use crate::heap::{
    FieldType, HeapFilterFlags, HeapIterationCallback, HeapVisitControlFlags, NonZeroJlong,
    PrimitiveArray, U16StrPrintable,
};
use crate::memory::{AllocatedArray, AllocatedMutf8, LocalRef};
use crate::util::*;
use core::ffi::c_void;
use jni::objects::JValue;
use jni_jvmti_sys::jvmtiEventMode::{JVMTI_DISABLE, JVMTI_ENABLE};
use jni_jvmti_sys::{
    jvmtiCapabilities, jvmtiEnv, jvmtiEventCallbacks, jvmtiHeapCallbacks, jvmtiHeapReferenceInfo,
    jvmtiHeapReferenceKind, jvmtiInterface_1_, jvmtiPrimitiveType, JVMTI_VERSION_1_1,
};
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use std::convert::TryFrom;
use std::os::raw::c_char;
use widestring::U16Str;

/// Shared across threads.
/// TODO how to dispose via RAII?
#[derive(Clone)]
#[repr(transparent)]
pub struct JvmtiEnv<'a>(*mut jvmtiEnv, PhantomData<&'a ()>);

// TODO expose a low-level direct api to jvmti, then a higher-level api for ergonomic use
//  e.g. capability builder that checks potential and automatically relinquishes/requests
//  direct(*mut jvmtiEnv), ergonomic(direct), ergonomic.into_inner()
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

    pub fn get_potential_capabilities(&self) -> JvmtiResult<jvmtiCapabilities> {
        let mut cap = MaybeUninit::<jvmtiCapabilities>::zeroed();
        jvmti_method!(self, GetPotentialCapabilities, cap.as_mut_ptr());
        let capabilities = unsafe { cap.assume_init() };
        debug!("potential capabilities: {:?}", capabilities);
        Ok(capabilities)
    }

    pub fn get_capabilities(&self) -> JvmtiResult<jvmtiCapabilities> {
        let mut cap = MaybeUninit::<jvmtiCapabilities>::zeroed();
        jvmti_method!(self, GetCapabilities, cap.as_mut_ptr());
        let capabilities = unsafe { cap.assume_init() };
        debug!("active capabilities: {:?}", capabilities);
        Ok(capabilities)
    }

    pub fn add_capabilities(&self, capabilities: &jvmtiCapabilities) -> JvmtiResult<()> {
        jvmti_method!(
            self,
            AddCapabilities,
            capabilities as *const jvmtiCapabilities
        );
        debug!("added capabilities {:?}", capabilities);
        Ok(())
    }

    pub fn relinquish_capabilities(&self, capabilities: &jvmtiCapabilities) -> JvmtiResult<()> {
        jvmti_method!(
            self,
            RelinquishCapabilities,
            capabilities as *const jvmtiCapabilities
        );
        debug!("relinquished capabilities {:?}", capabilities);
        Ok(())
    }

    pub fn iterate_through_heap(
        &self,
        heap_filter: HeapFilterFlags,
        instanceof: Option<jclass>,
        mut callback: impl FnMut(HeapIterationCallback) -> HeapVisitControlFlags,
    ) -> JvmtiResult<()> {
        unsafe extern "C" fn heap_iteration_callback(
            class_tag: jlong,
            size: jlong,
            tag_ptr: *mut jlong,
            length: jint,
            user_data: *mut c_void,
        ) -> jint {
            let closure: &mut &mut dyn FnMut(HeapIterationCallback) -> HeapVisitControlFlags =
                &mut *(user_data as *mut &mut _);

            let class_tag = NonZeroJlong::new(class_tag);
            debug_assert!(size >= 0);
            let tag = tag_ptr.as_mut().expect("tag pointer is null");
            let array_length = if length < 0 {
                None
            } else {
                Some(length as usize)
            };
            let arg = HeapIterationCallback::Object {
                class_tag,
                size: size as usize,
                tag,
                array_length,
            };
            closure(arg).bits()
        }

        unsafe extern "C" fn primitive_field_callback(
            kind: jvmtiHeapReferenceKind,
            info: *const jvmtiHeapReferenceInfo,
            object_class_tag: jlong,
            object_tag_ptr: *mut jlong,
            value: jvalue,
            value_type: jvmtiPrimitiveType,
            user_data: *mut ::core::ffi::c_void,
        ) -> jint {
            use jvmtiHeapReferenceKind::*;
            use jvmtiPrimitiveType::*;

            let closure: &mut &mut dyn FnMut(HeapIterationCallback) -> HeapVisitControlFlags =
                &mut *(user_data as *mut &mut _);

            let field_type = match kind {
                JVMTI_HEAP_REFERENCE_FIELD => FieldType::Instance,
                JVMTI_HEAP_REFERENCE_STATIC_FIELD => FieldType::Static,
                _ => unreachable!("unexpected field kind {:?}", kind),
            };

            let field_index = (*info).field.index;
            let object_class_tag = NonZeroJlong::new(object_class_tag);
            let object_tag = object_tag_ptr.as_mut().expect("tag pointer is null");
            let value = match value_type {
                JVMTI_PRIMITIVE_TYPE_BOOLEAN => JValue::Bool(value.z),
                JVMTI_PRIMITIVE_TYPE_BYTE => JValue::Byte(value.b),
                JVMTI_PRIMITIVE_TYPE_CHAR => JValue::Char(value.c),
                JVMTI_PRIMITIVE_TYPE_SHORT => JValue::Short(value.s),
                JVMTI_PRIMITIVE_TYPE_INT => JValue::Int(value.i),
                JVMTI_PRIMITIVE_TYPE_LONG => JValue::Long(value.j),
                JVMTI_PRIMITIVE_TYPE_FLOAT => JValue::Float(value.f),
                JVMTI_PRIMITIVE_TYPE_DOUBLE => JValue::Double(value.d),
            };

            let arg = HeapIterationCallback::PrimitiveField {
                field_type,
                field_index,
                object_class_tag,
                object_tag,
                value,
            };
            closure(arg).bits()
        }

        unsafe extern "C" fn primitive_array_callback(
            class_tag: jlong,
            size: jlong,
            tag_ptr: *mut jlong,
            element_count: jint,
            element_type: jvmtiPrimitiveType,
            elements: *const c_void,
            user_data: *mut c_void,
        ) -> jint {
            let closure: &mut &mut dyn FnMut(HeapIterationCallback) -> HeapVisitControlFlags =
                &mut *(user_data as *mut &mut _);

            let class_tag = NonZeroJlong::new(class_tag);
            debug_assert!(size >= 0);
            let tag = tag_ptr.as_mut().expect("tag pointer is null");
            debug_assert!(element_count >= 0);
            let elements =
                PrimitiveArray::new(elements as *const _, element_count as usize, element_type);

            let arg = HeapIterationCallback::PrimitiveArray {
                class_tag,
                size: size as usize,
                tag,
                elements,
            };
            closure(arg).bits()
        }

        unsafe extern "C" fn string_callback(
            class_tag: jlong,
            size: jlong,
            tag_ptr: *mut jlong,
            value: *const jchar,
            value_length: jint,
            user_data: *mut ::core::ffi::c_void,
        ) -> jint {
            let closure: &mut &mut dyn FnMut(HeapIterationCallback) -> HeapVisitControlFlags =
                &mut *(user_data as *mut &mut _);

            let class_tag = NonZeroJlong::new(class_tag);
            debug_assert!(size >= 0);
            let tag = tag_ptr.as_mut().expect("tag pointer is null");
            debug_assert!(value_length >= 0);
            let value = U16StrPrintable(U16Str::from_ptr(value, value_length as usize));

            let arg = HeapIterationCallback::String {
                class_tag,
                size: size as usize,
                tag,
                value,
            };
            closure(arg).bits()
        }

        let raw_callbacks = jvmtiHeapCallbacks {
            heap_iteration_callback: Some(heap_iteration_callback),
            heap_reference_callback: None,
            primitive_field_callback: Some(primitive_field_callback),
            array_primitive_value_callback: Some(primitive_array_callback),
            string_primitive_value_callback: Some(string_callback),
            ..Default::default()
        };

        let mut callback: &mut dyn FnMut(_) -> _ = &mut callback;
        let callback = &mut callback;
        debug!("iterating over heap");
        jvmti_method!(
            self,
            IterateThroughHeap,
            heap_filter.bits(),
            instanceof.unwrap_or(null_mut()),
            &raw_callbacks as *const jvmtiHeapCallbacks,
            callback as *mut _ as *mut c_void
        );

        Ok(())
    }

    pub fn get_objects_with_tag(
        &self,
        tag: jlong,
        jni: jni::JNIEnv<'a>,
    ) -> JvmtiResult<AllocatedArray<LocalRef>> {
        let tags = [tag];
        self.get_objects_with_tags(&tags, jni)
    }

    // TODO generic param to also return array of tag results
    pub fn get_objects_with_tags(
        &self,
        tags: &[jlong],
        jni: jni::JNIEnv<'a>,
    ) -> JvmtiResult<AllocatedArray<LocalRef>> {
        let tag_count = jint::try_from(tags.len()).expect("too many tags)");

        let mut obj_count: jint = 0;
        let mut obj_array = null_mut();
        jvmti_method!(
            self,
            GetObjectsWithTags,
            tag_count,
            tags.as_ptr(),
            &mut obj_count as *mut jint,
            (&mut obj_array) as *mut *mut jobject,
            null_mut()
        );

        Ok(unsafe {
            AllocatedArray::<LocalRef>::new(obj_array, obj_count as usize, jni, self.clone())
        })
    }

    // TODO generic param to optionally get generic signature too
    pub fn get_class_signature(&self, class: jclass) -> JvmtiResult<AllocatedMutf8> {
        let mut jni_sig: *mut c_char = null_mut();
        jvmti_method!(
            self,
            GetClassSignature,
            class,
            (&mut jni_sig) as *mut *mut c_char,
            null_mut()
        );

        assert!(!jni_sig.is_null());
        Ok(unsafe { AllocatedMutf8::new(jni_sig, self.clone()) })
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
