#![allow(non_snake_case)]
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
    pub fn new(s: &str) -> Self {
        JavaVMOption {
            extraInfo: std::ptr::null_mut(),
            optionString: CString::new(s).unwrap().into_raw(),
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
    /// 创建一个包含 Java 字符串的 `jvalue`。
    ///
    /// # Safety
    /// - `pjenv` 必须是一个有效的 `JNIEnv` 指针。
    /// - `s` 必须是一个有效的 UTF-8 字符串。
    /// - 调用者必须确保 `pjenv` 指向的 JNI 环境是有效的，并且当前线程已附加到 JVM。
    /// - 返回的 `jstring` 是一个本地引用，调用者需要确保在调用后正确管理其生命周期。
    pub unsafe fn str(pjenv: *mut JNIEnv, s: &str) -> jvalue {
        jvalue {
            l: unsafe { (*pjenv).NewStringUTF(CString::new(s).unwrap().as_ptr()).unwrap() as jobject },
        }
    }

    /// 创建一个包含 `jint` 的 `jvalue`。
    pub fn jint(i: jint) -> jvalue {
        jvalue { i }
    }

    /// 创建一个包含 `jlong` 的 `jvalue`。
    pub fn jlong(j: jlong) -> jvalue {
        jvalue { j }
    }

    /// 创建一个空的 `jvalue`（表示 `null`）。
    pub fn null() -> jvalue {
        jvalue { l: null_mut() }
    }
}
