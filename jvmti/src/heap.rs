use jni::objects::JValue;
use jni::sys::*;
use jni_jvmti_sys::*;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::num::NonZeroI64;

pub type NonZeroJlong = NonZeroI64;

bitflags::bitflags! {
    pub struct HeapFilterFlags : jint {
        const TAGGED = JVMTI_HEAP_FILTER_TAGGED as _;
        const UNTAGGED = JVMTI_HEAP_FILTER_UNTAGGED as _;
        const CLASS_TAGGED = JVMTI_HEAP_FILTER_CLASS_TAGGED as _;
        const CLASS_UNTAGGED = JVMTI_HEAP_FILTER_CLASS_UNTAGGED as _;
    }
}

bitflags::bitflags! {
    pub struct HeapVisitControlFlags : jint {
        const VISIT_OBJECTS = JVMTI_VISIT_OBJECTS as _;
        const ABORT = JVMTI_VISIT_ABORT as _;
    }
}

#[derive(Debug)]
pub enum FieldType {
    Instance,
    Static,
}

pub struct U16StrPrintable<'a>(pub &'a widestring::U16Str);

pub struct PrimitiveArray<'a> {
    ptr: *const (),
    len: usize,
    ty: jvmtiPrimitiveType,
    phantom: PhantomData<&'a ()>,
}

/// Callback for `IterateThroughHeap`
#[derive(Debug)]
pub enum HeapIterationCallback<'a> {
    Object {
        class_tag: Option<NonZeroJlong>,
        size: usize,
        tag: &'a mut jlong,
        /// None for non-arrays
        array_length: Option<usize>,
    },

    PrimitiveField {
        field_type: FieldType,
        field_index: jint,
        object_class_tag: Option<NonZeroJlong>,
        object_tag: &'a mut jlong,
        value: JValue<'a>,
    },

    String {
        class_tag: Option<NonZeroJlong>,
        /// Bytes
        size: usize,
        tag: &'a mut jlong,
        value: U16StrPrintable<'a>,
    },

    PrimitiveArray {
        class_tag: Option<NonZeroJlong>,
        /// Bytes
        size: usize,
        tag: &'a mut jlong,
        elements: PrimitiveArray<'a>,
    },
}

macro_rules! try_slice {
    ($name:ident, $ty:ty, $prim_type:ident) => {
        pub fn $name(&self) -> Option<&[$ty]> {
            if let jvmtiPrimitiveType::$prim_type = self.ty {
                Some(unsafe { std::slice::from_raw_parts(self.ptr as *const $ty, self.len) })
            } else {
                None
            }
        }
    };
}
impl<'a> PrimitiveArray<'a> {
    pub fn new(ptr: *const (), len: usize, ty: jvmtiPrimitiveType) -> Self {
        Self {
            ptr,
            len,
            ty,
            phantom: PhantomData,
        }
    }

    try_slice!(z, jboolean, JVMTI_PRIMITIVE_TYPE_BOOLEAN);
    try_slice!(b, jbyte, JVMTI_PRIMITIVE_TYPE_BYTE);
    try_slice!(c, jchar, JVMTI_PRIMITIVE_TYPE_CHAR);
    try_slice!(s, jshort, JVMTI_PRIMITIVE_TYPE_SHORT);
    try_slice!(i, jint, JVMTI_PRIMITIVE_TYPE_INT);
    try_slice!(j, jlong, JVMTI_PRIMITIVE_TYPE_LONG);
    try_slice!(f, jfloat, JVMTI_PRIMITIVE_TYPE_FLOAT);
    try_slice!(d, jdouble, JVMTI_PRIMITIVE_TYPE_DOUBLE);
}
impl Debug for PrimitiveArray<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const LIMIT: usize = 64;
        struct LimitedArray<'a, T>(&'a [T]);
        impl<T: Debug> Debug for LimitedArray<'_, T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let mut list = f.debug_list();
                list.entries(self.0.iter().take(LIMIT));
                if self.0.len() > LIMIT {
                    list.entry(&"..");
                }

                list.finish()
            }
        }

        if let Some(array) = self.z() {
            write!(f, "jboolean[{}] => {:?}", array.len(), LimitedArray(array))
        } else if let Some(array) = self.b() {
            write!(f, "jbyte[{}] => {:?}", array.len(), LimitedArray(array))
        } else if let Some(array) = self.c() {
            write!(f, "jchar[{}] => {:?}", array.len(), LimitedArray(array))
        } else if let Some(array) = self.s() {
            write!(f, "jshort[{}] => {:?}", array.len(), LimitedArray(array))
        } else if let Some(array) = self.i() {
            write!(f, "jint[{}] => {:?}", array.len(), LimitedArray(array))
        } else if let Some(array) = self.j() {
            write!(f, "jlong[{}] => {:?}", array.len(), LimitedArray(array))
        } else if let Some(array) = self.f() {
            write!(f, "jfloat[{}] => {:?}", array.len(), LimitedArray(array))
        } else if let Some(array) = self.d() {
            write!(f, "jdouble[{}] => {:?}", array.len(), LimitedArray(array))
        } else {
            unreachable!()
        }
    }
}

impl Debug for U16StrPrintable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self.0.to_string_lossy();
        write!(f, "{:?}", str)
    }
}
