use core::fmt::{Debug, Formatter};
use jni_jvmti_sys::*;
use std::mem::transmute;

#[derive(Default)]
pub struct EventCallbacksBuilder {
    vminit: jvmtiEventVMInit,
    vmdeath: jvmtiEventVMDeath,
    thread_start: jvmtiEventThreadStart,
    thread_end: jvmtiEventThreadEnd,
    class_file_load_hook: jvmtiEventClassFileLoadHook,
    class_load: jvmtiEventClassLoad,
    class_prepare: jvmtiEventClassPrepare,
    vmstart: jvmtiEventVMStart,
    exception: jvmtiEventException,
    exception_catch: jvmtiEventExceptionCatch,
    single_step: jvmtiEventSingleStep,
    frame_pop: jvmtiEventFramePop,
    breakpoint: jvmtiEventBreakpoint,
    field_access: jvmtiEventFieldAccess,
    field_modification: jvmtiEventFieldModification,
    method_entry: jvmtiEventMethodEntry,
    method_exit: jvmtiEventMethodExit,
    native_method_bind: jvmtiEventNativeMethodBind,
    compiled_method_load: jvmtiEventCompiledMethodLoad,
    compiled_method_unload: jvmtiEventCompiledMethodUnload,
    dynamic_code_generated: jvmtiEventDynamicCodeGenerated,
    data_dump_request: jvmtiEventDataDumpRequest,
    monitor_wait: jvmtiEventMonitorWait,
    monitor_waited: jvmtiEventMonitorWaited,
    monitor_contended_enter: jvmtiEventMonitorContendedEnter,
    monitor_contended_entered: jvmtiEventMonitorContendedEntered,
    resource_exhausted: jvmtiEventResourceExhausted,
    garbage_collection_start: jvmtiEventGarbageCollectionStart,
    garbage_collection_finish: jvmtiEventGarbageCollectionFinish,
    object_free: jvmtiEventObjectFree,
    vmobject_alloc: jvmtiEventVMObjectAlloc,
    sampled_object_alloc: jvmtiEventSampledObjectAlloc,
}
#[repr(transparent)]
pub struct EventCallbacks(jvmtiEventCallbacks);

#[derive(Copy, Clone, Debug)]
pub enum EventType {
    VmDeath = 51,
    ThreadStart = 52,
    ThreadEnd = 53,
    ClassFileLoadHook = 54,
    ClassLoad = 55,
    ClassPrepare = 56,
    VmStart = 57,
    Exception = 58,
    ExceptionCatch = 59,
    SingleStep = 60,
    FramePop = 61,
    Breakpoint = 62,
    FieldAccess = 63,
    FieldModification = 64,
    MethodEntry = 65,
    MethodExit = 66,
    NativeMethodBind = 67,
    CompiledMethodLoad = 68,
    CompiledMethodUnload = 69,
    DynamicCodeGenerated = 70,
    DataDumpRequest = 71,
    MonitorWait = 73,
    MonitorWaited = 74,
    MonitorContendedEnter = 75,
    MonitorContendedEntered = 76,
    ResourceExhausted = 80,
    GarbageCollectionStart = 81,
    GarbageCollectionFinish = 82,
    ObjectFree = 83,
    VmObjectAlloc = 84,
    SampledObjectAlloc = 86,
}
#[derive(Copy, Clone, Debug)]
pub enum EventScope {
    Global,
    Thread(jthread), // TODO thread wrapper that knows its pid, and format that in Debug
}

impl From<&EventCallbacks> for *const jvmtiEventCallbacks {
    fn from(callbacks: &EventCallbacks) -> Self {
        &callbacks.0 as *const _
    }
}

impl EventCallbacksBuilder {
    pub fn build(self) -> EventCallbacks {
        EventCallbacks(jvmtiEventCallbacks {
            VMInit: self.vminit,
            VMDeath: self.vmdeath,
            ThreadStart: self.thread_start,
            ThreadEnd: self.thread_end,
            ClassFileLoadHook: self.class_file_load_hook,
            ClassLoad: self.class_load,
            ClassPrepare: self.class_prepare,
            VMStart: self.vmstart,
            Exception: self.exception,
            ExceptionCatch: self.exception_catch,
            SingleStep: self.single_step,
            FramePop: self.frame_pop,
            Breakpoint: self.breakpoint,
            FieldAccess: self.field_access,
            FieldModification: self.field_modification,
            MethodEntry: self.method_entry,
            MethodExit: self.method_exit,
            NativeMethodBind: self.native_method_bind,
            CompiledMethodLoad: self.compiled_method_load,
            CompiledMethodUnload: self.compiled_method_unload,
            DynamicCodeGenerated: self.dynamic_code_generated,
            DataDumpRequest: self.data_dump_request,
            reserved72: None,
            MonitorWait: self.monitor_wait,
            MonitorWaited: self.monitor_waited,
            MonitorContendedEnter: self.monitor_contended_enter,
            MonitorContendedEntered: self.monitor_contended_entered,
            reserved77: None,
            reserved78: None,
            reserved79: None,
            ResourceExhausted: self.resource_exhausted,
            GarbageCollectionStart: self.garbage_collection_start,
            GarbageCollectionFinish: self.garbage_collection_finish,
            ObjectFree: self.object_free,
            VMObjectAlloc: self.vmobject_alloc,
            reserved85: None,
            SampledObjectAlloc: self.sampled_object_alloc,
        })
    }

    pub fn with_vminit(mut self, callback: jvmtiEventVMInit) -> Self {
        self.vminit = callback;
        self
    }

    pub fn with_vmdeath(mut self, callback: Option<callback_types::VMDeath>) -> Self {
        self.vmdeath = callback.map(|ptr| unsafe { transmute(ptr) });
        self
    }

    pub fn with_thread_start(mut self, callback: jvmtiEventThreadStart) -> Self {
        self.thread_start = callback;
        self
    }

    pub fn with_thread_end(mut self, callback: jvmtiEventThreadEnd) -> Self {
        self.thread_end = callback;
        self
    }

    pub fn with_class_file_load_hook(mut self, callback: jvmtiEventClassFileLoadHook) -> Self {
        self.class_file_load_hook = callback;
        self
    }

    pub fn with_class_load(mut self, callback: Option<callback_types::ClassLoad>) -> Self {
        self.class_load = callback.map(|ptr| unsafe { transmute(ptr) });
        self
    }

    pub fn with_class_prepare(mut self, callback: jvmtiEventClassPrepare) -> Self {
        self.class_prepare = callback;
        self
    }

    pub fn with_vmstart(mut self, callback: jvmtiEventVMStart) -> Self {
        self.vmstart = callback;
        self
    }

    pub fn with_exception(mut self, callback: jvmtiEventException) -> Self {
        self.exception = callback;
        self
    }

    pub fn with_exception_catch(mut self, callback: jvmtiEventExceptionCatch) -> Self {
        self.exception_catch = callback;
        self
    }

    pub fn with_single_step(mut self, callback: jvmtiEventSingleStep) -> Self {
        self.single_step = callback;
        self
    }

    pub fn with_frame_pop(mut self, callback: jvmtiEventFramePop) -> Self {
        self.frame_pop = callback;
        self
    }

    pub fn with_breakpoint(mut self, callback: jvmtiEventBreakpoint) -> Self {
        self.breakpoint = callback;
        self
    }

    pub fn with_field_access(mut self, callback: jvmtiEventFieldAccess) -> Self {
        self.field_access = callback;
        self
    }

    pub fn with_field_modification(mut self, callback: jvmtiEventFieldModification) -> Self {
        self.field_modification = callback;
        self
    }

    pub fn with_method_entry(mut self, callback: jvmtiEventMethodEntry) -> Self {
        self.method_entry = callback;
        self
    }

    pub fn with_method_exit(mut self, callback: jvmtiEventMethodExit) -> Self {
        self.method_exit = callback;
        self
    }

    pub fn with_native_method_bind(mut self, callback: jvmtiEventNativeMethodBind) -> Self {
        self.native_method_bind = callback;
        self
    }

    pub fn with_compiled_method_load(mut self, callback: jvmtiEventCompiledMethodLoad) -> Self {
        self.compiled_method_load = callback;
        self
    }

    pub fn with_compiled_method_unload(mut self, callback: jvmtiEventCompiledMethodUnload) -> Self {
        self.compiled_method_unload = callback;
        self
    }

    pub fn with_dynamic_code_generated(mut self, callback: jvmtiEventDynamicCodeGenerated) -> Self {
        self.dynamic_code_generated = callback;
        self
    }

    pub fn with_data_dump_request(mut self, callback: jvmtiEventDataDumpRequest) -> Self {
        self.data_dump_request = callback;
        self
    }

    pub fn with_monitor_wait(mut self, callback: jvmtiEventMonitorWait) -> Self {
        self.monitor_wait = callback;
        self
    }

    pub fn with_monitor_waited(mut self, callback: jvmtiEventMonitorWaited) -> Self {
        self.monitor_waited = callback;
        self
    }

    pub fn with_monitor_contended_enter(
        mut self,
        callback: jvmtiEventMonitorContendedEnter,
    ) -> Self {
        self.monitor_contended_enter = callback;
        self
    }

    pub fn with_monitor_contended_entered(
        mut self,
        callback: jvmtiEventMonitorContendedEntered,
    ) -> Self {
        self.monitor_contended_entered = callback;
        self
    }

    pub fn with_resource_exhausted(mut self, callback: jvmtiEventResourceExhausted) -> Self {
        self.resource_exhausted = callback;
        self
    }

    pub fn with_garbage_collection_start(
        mut self,
        callback: jvmtiEventGarbageCollectionStart,
    ) -> Self {
        self.garbage_collection_start = callback;
        self
    }

    pub fn with_garbage_collection_finish(
        mut self,
        callback: jvmtiEventGarbageCollectionFinish,
    ) -> Self {
        self.garbage_collection_finish = callback;
        self
    }

    pub fn with_object_free(mut self, callback: jvmtiEventObjectFree) -> Self {
        self.object_free = callback;
        self
    }

    pub fn with_vmobject_alloc(mut self, callback: jvmtiEventVMObjectAlloc) -> Self {
        self.vmobject_alloc = callback;
        self
    }

    pub fn with_sampled_object_alloc(mut self, callback: jvmtiEventSampledObjectAlloc) -> Self {
        self.sampled_object_alloc = callback;
        self
    }
}
impl Debug for EventCallbacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut list = f.debug_list();

        struct CallbackDebug(&'static str, *const ());
        impl Debug for CallbackDebug {
            fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
                write!(f, "{} ({:?})", self.0, self.1)
            }
        }
        macro_rules! callback {
            ($name:ident) => {
                if let Some(ptr) = self.0. $name {
                    list.entry(&CallbackDebug(stringify!($name), ptr as *const ()));
                }
            };
        }
        callback!(VMInit);
        callback!(VMDeath);
        callback!(ThreadStart);
        callback!(ThreadEnd);
        callback!(ClassFileLoadHook);
        callback!(ClassLoad);
        callback!(ClassPrepare);
        callback!(VMStart);
        callback!(Exception);
        callback!(ExceptionCatch);
        callback!(SingleStep);
        callback!(FramePop);
        callback!(Breakpoint);
        callback!(FieldAccess);
        callback!(FieldModification);
        callback!(MethodEntry);
        callback!(MethodExit);
        callback!(NativeMethodBind);
        callback!(CompiledMethodLoad);
        callback!(CompiledMethodUnload);
        callback!(DynamicCodeGenerated);
        callback!(DataDumpRequest);
        callback!(MonitorWait);
        callback!(MonitorWaited);
        callback!(MonitorContendedEnter);
        callback!(MonitorContendedEntered);
        callback!(ResourceExhausted);
        callback!(GarbageCollectionStart);
        callback!(GarbageCollectionFinish);
        callback!(ObjectFree);
        callback!(VMObjectAlloc);
        callback!(SampledObjectAlloc);

        list.finish()
    }
}

impl From<EventType> for jvmtiEvent {
    fn from(evt: EventType) -> Self {
        use jvmtiEvent::*;
        use EventType::*;
        match evt {
            VmDeath => JVMTI_EVENT_VM_DEATH,
            ThreadStart => JVMTI_EVENT_THREAD_START,
            ThreadEnd => JVMTI_EVENT_THREAD_END,
            ClassFileLoadHook => JVMTI_EVENT_CLASS_FILE_LOAD_HOOK,
            ClassLoad => JVMTI_EVENT_CLASS_LOAD,
            ClassPrepare => JVMTI_EVENT_CLASS_PREPARE,
            VmStart => JVMTI_EVENT_VM_START,
            Exception => JVMTI_EVENT_EXCEPTION,
            ExceptionCatch => JVMTI_EVENT_EXCEPTION_CATCH,
            SingleStep => JVMTI_EVENT_SINGLE_STEP,
            FramePop => JVMTI_EVENT_FRAME_POP,
            Breakpoint => JVMTI_EVENT_BREAKPOINT,
            FieldAccess => JVMTI_EVENT_FIELD_ACCESS,
            FieldModification => JVMTI_EVENT_FIELD_MODIFICATION,
            MethodEntry => JVMTI_EVENT_METHOD_ENTRY,
            MethodExit => JVMTI_EVENT_METHOD_EXIT,
            NativeMethodBind => JVMTI_EVENT_NATIVE_METHOD_BIND,
            CompiledMethodLoad => JVMTI_EVENT_COMPILED_METHOD_LOAD,
            CompiledMethodUnload => JVMTI_EVENT_COMPILED_METHOD_UNLOAD,
            DynamicCodeGenerated => JVMTI_EVENT_DYNAMIC_CODE_GENERATED,
            DataDumpRequest => JVMTI_EVENT_DATA_DUMP_REQUEST,
            MonitorWait => JVMTI_EVENT_MONITOR_WAIT,
            MonitorWaited => JVMTI_EVENT_MONITOR_WAITED,
            MonitorContendedEnter => JVMTI_EVENT_MONITOR_CONTENDED_ENTER,
            MonitorContendedEntered => JVMTI_EVENT_MONITOR_CONTENDED_ENTERED,
            ResourceExhausted => JVMTI_EVENT_RESOURCE_EXHAUSTED,
            GarbageCollectionStart => JVMTI_EVENT_GARBAGE_COLLECTION_START,
            GarbageCollectionFinish => JVMTI_EVENT_GARBAGE_COLLECTION_FINISH,
            ObjectFree => JVMTI_EVENT_OBJECT_FREE,
            VmObjectAlloc => JVMTI_EVENT_VM_OBJECT_ALLOC,
            SampledObjectAlloc => JVMTI_EVENT_SAMPLED_OBJECT_ALLOC,
        }
    }
}

//noinspection ALL
mod callback_types {
    #![allow(dead_code)]
    // TODO more nicer callback types
    use jni::sys::*;
    use jni_jvmti_sys::*;

    pub type Breakpoint = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
    );

    pub type ClassFileLoadHook = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        class_being_redefined: jclass,
        loader: jobject,
        name: *const ::std::os::raw::c_char,
        protection_domain: jobject,
        class_data_len: jint,
        class_data: *const ::std::os::raw::c_uchar,
        new_class_data_len: *mut jint,
        new_class_data: *mut *mut ::std::os::raw::c_uchar,
    );

    pub type ClassLoad = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        klass: jclass,
    );

    pub type ClassPrepare = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        klass: jclass,
    );

    pub type CompiledMethodLoad = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        method: jmethodID,
        code_size: jint,
        code_addr: *const ::core::ffi::c_void,
        map_length: jint,
        map: *const jvmtiAddrLocationMap,
        compile_info: *const ::core::ffi::c_void,
    );

    pub type CompiledMethodUnload = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        method: jmethodID,
        code_addr: *const ::core::ffi::c_void,
    );

    pub type DataDumpRequest = for<'a> unsafe extern "C" fn(jvmti_env: crate::env::JvmtiEnv);

    pub type DynamicCodeGenerated = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        name: *const ::std::os::raw::c_char,
        address: *const ::core::ffi::c_void,
        length: jint,
    );

    pub type Exception = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        exception: jobject,
        catch_method: jmethodID,
        catch_location: jlocation,
    );

    pub type ExceptionCatch = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        exception: jobject,
    );

    pub type FieldAccess = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        field_klass: jclass,
        object: jobject,
        field: jfieldID,
    );

    pub type FieldModification = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        field_klass: jclass,
        object: jobject,
        field: jfieldID,
        signature_type: ::std::os::raw::c_char,
        new_value: jvalue,
    );

    pub type FramePop = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
        was_popped_by_exception: jboolean,
    );

    pub type GarbageCollectionFinish =
        for<'a> unsafe extern "C" fn(jvmti_env: crate::env::JvmtiEnv);

    pub type GarbageCollectionStart = for<'a> unsafe extern "C" fn(jvmti_env: crate::env::JvmtiEnv);

    pub type MethodEntry = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
    );

    pub type MethodExit = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
        was_popped_by_exception: jboolean,
        return_value: jvalue,
    );

    pub type MonitorContendedEnter = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        object: jobject,
    );

    pub type MonitorContendedEntered = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        object: jobject,
    );

    pub type MonitorWait = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        object: jobject,
        timeout: jlong,
    );

    pub type MonitorWaited = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        object: jobject,
        timed_out: jboolean,
    );

    pub type NativeMethodBind = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
        address: *mut ::core::ffi::c_void,
        new_address_ptr: *mut *mut ::core::ffi::c_void,
    );

    pub type ObjectFree = for<'a> unsafe extern "C" fn(jvmti_env: crate::env::JvmtiEnv, tag: jlong);

    pub type ResourceExhausted = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        flags: jint,
        reserved: *const ::core::ffi::c_void,
        description: *const ::std::os::raw::c_char,
    );

    pub type SampledObjectAlloc = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        object: jobject,
        object_klass: jclass,
        size: jlong,
    );

    pub type SingleStep = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
    );

    pub type ThreadEnd = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
    );

    pub type ThreadStart = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
    );

    pub type VMDeath =
        for<'a> unsafe extern "C" fn(jvmti_env: crate::env::JvmtiEnv, jni_env: jni::JNIEnv<'a>);

    pub type VMInit = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
    );

    pub type VMObjectAlloc = for<'a> unsafe extern "C" fn(
        jvmti_env: crate::env::JvmtiEnv,
        jni_env: jni::JNIEnv<'a>,
        thread: jthread,
        object: jobject,
        object_klass: jclass,
        size: jlong,
    );

    pub type VMStart =
        for<'a> unsafe extern "C" fn(jvmti_env: crate::env::JvmtiEnv, jni_env: jni::JNIEnv<'a>);
}
