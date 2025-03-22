#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#[macro_use]
mod macros;
use std::{
    collections::HashMap,
    ffi::CString,
    ops::{Deref, DerefMut},
    os::raw::c_void,
    ptr::null_mut,
};
mod JNI;
use JNI::{
    JNI_CreateJavaVM, JNI_FALSE, JNI_OK, JNI_TRUE, JNI_VERSION_21, JNIEnv, JavaVM, JavaVMInitArgs,
    JavaVMOption, jclass, jfieldID, jint, jmethodID, jobject, jstring, jvalue, va_list,
};
mod ClassWrap;
use ClassWrap::{Counter, Person};
mod JNIWrap;
use JNIWrap::{CreateJavaWrapper, JavaVMWrapper};

fn main() -> ::std::io::Result<()> {
    let optionString = c!(r"-Djava.class.path=.");
    let mut options = JavaVMOption {
        extraInfo: std::ptr::null_mut(),
        optionString,
    };
    let mut vm_args: JavaVMInitArgs = JavaVMInitArgs {
        version: JNI_VERSION_21 as i32,
        nOptions: 1,
        options: &mut options,
        ignoreUnrecognized: JNI_FALSE as u8,
    };

    let (jvm, jenv) = CreateJavaWrapper(vm_args);

    Ok(())
}
