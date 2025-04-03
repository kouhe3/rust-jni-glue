#![allow(non_snake_case)]
macro_rules! c {
    ($s:expr) => {
        CString::new($s).unwrap().into_raw()
    };
}
use jnimacro::jni_method;
pub mod JNI;

use JNI::{
    JNI_CreateJavaVM, JNIEnv, JavaVM, JavaVMInitArgs, JavaVMOption, jclass, jfieldID, jint, jlong,
    jmethodID, jobject, jstring, jvalue,
};
use std::{ffi::CString, os::raw::c_void, ptr::null_mut};

pub fn CreateJavaWrapper(mut vm_args: JavaVMInitArgs) -> (*mut JavaVM, *mut JNIEnv) {
    let mut pjvm = null_mut::<JavaVM>();
    let mut pjenv = null_mut::<JNIEnv>();
    unsafe {
        JNI_CreateJavaVM(
            &mut pjvm,
            &mut pjenv as *mut *mut JNIEnv as *mut *mut c_void,
            &mut vm_args as *mut JavaVMInitArgs as *mut c_void,
        );

        (pjvm, pjenv)
    }
}
impl JavaVMOption {
    pub fn new(optionString: &str) -> Self {
        JavaVMOption {
            extraInfo: std::ptr::null_mut(),
            optionString: c!(optionString),
        }
    }
}
impl JavaVMInitArgs {
    pub fn new(
        version: u32,
        nOptions: i32,
        options: &mut JavaVMOption,
        ignoreUnrecognized: bool,
    ) -> Self {
        JavaVMInitArgs {
            version: version as i32,
            nOptions,
            options,
            ignoreUnrecognized: ignoreUnrecognized as u8,
        }
    }
}

impl JNIEnv {
    jni_method!(NewObjectA: (jclass, jmethodID, *const jvalue) -> Option<jobject>  );
    jni_method!(FindClass: (*const ::std::os::raw::c_char) -> Option<jclass>);
    jni_method!(NewStringUTF: (*const ::std::os::raw::c_char) -> Option<jstring>);
    jni_method!(CallStaticVoidMethodA: (jclass, jmethodID, *const jvalue) -> Option<()>);
    jni_method!(CallStaticIntMethodA: (jclass, jmethodID, *const jvalue) -> Option<jint>);
    jni_method!(CallVoidMethodA: (jobject, jmethodID, *const jvalue) -> Option<()>);
    jni_method!(GetStaticMethodID: (jclass, *const ::std::os::raw::c_char, *const ::std::os::raw::c_char) -> Option<jmethodID>);
    jni_method!(GetMethodID: (jclass, *const ::std::os::raw::c_char, *const ::std::os::raw::c_char) -> Option<jmethodID>);
    jni_method!(GetObjectField: (jobject, jfieldID) -> Option<jobject>);
    jni_method!(GetObjectClass: (jobject) -> Option<jclass>);
}

impl JavaVM {
    jni_method!(DestroyJavaVM: () -> Option<i32>);
}

impl jvalue {
    pub unsafe fn str(pjenv: *mut JNIEnv, s: &str) -> jvalue {
        jvalue {
            l: unsafe { (*pjenv).NewStringUTF(c!(s)).unwrap() as jobject },
        }
    }
    pub fn jint(i: jint) -> jvalue {
        jvalue { i }
    }
    pub fn jlong(j: jlong) -> jvalue {
        jvalue { j }
    }
    pub fn null() -> jvalue {
        jvalue { l: null_mut() }
    }
}
