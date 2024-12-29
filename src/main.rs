#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi::CString, os::raw::c_void, ptr::null_mut};
include!(concat!(env!("OUT_DIR"), "/bindgens.rs"));

fn main() {
    let optionString = CString::new("").unwrap().into_raw();
    let mut options = JavaVMOption {
        extraInfo: std::ptr::null_mut(),
        optionString,
    };
    let mut vm_args: JavaVMInitArgs = JavaVMInitArgs {
        version: JNI_VERSION_21 as i32,
        nOptions: 1,
        options: &mut options,
        ignoreUnrecognized: JNI_TRUE as u8,
    };
    let mut jvm = null_mut::<JavaVM>();
    let mut jenv = null_mut::<JNIEnv>();
    unsafe {
        JNI_CreateJavaVM(
            &mut jvm,
            &mut jenv as *mut *mut JNIEnv as *mut *mut c_void,
            &mut vm_args as *mut JavaVMInitArgs as *mut c_void,
        );
        let jenv_fn = jenv.as_ref().unwrap().functions.as_ref().unwrap();
        let jvm_fn = jvm.as_ref().unwrap().functions.as_ref().unwrap();
        let NewStringUTF = jenv_fn.NewStringUTF.unwrap();
        let system_class = jenv_fn.FindClass.unwrap()(jenv, c("java/lang/System"));
        let outField = jenv_fn.GetStaticFieldID.unwrap()(
            jenv,
            system_class,
            c("out"),
            c("Ljava/io/PrintStream;"),
        );
        let outObj = jenv_fn.GetStaticObjectField.unwrap()(jenv, system_class, outField);
        let printStreamClass = jenv_fn.FindClass.unwrap()(jenv, c("java/io/PrintStream"));
        let printlnMethod = jenv_fn.GetMethodID.unwrap()(
            jenv,
            printStreamClass,
            c("println"),
            c("(Ljava/lang/String;)V"),
        );
        let msg = NewStringUTF(jenv, c("Hello World"));
        jenv_fn.CallVoidMethod.unwrap()(jenv, outObj, printlnMethod, msg);
        jvm_fn.DestroyJavaVM.unwrap()(jvm);
    }
}

fn c(s: &str) -> *const i8 {
    CString::new(s).unwrap().into_raw()
}
