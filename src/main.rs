#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(c_variadic)]
#[macro_use]
mod macros;
use std::{ffi::CString, os::raw::c_void, ptr::null_mut};
mod JNI;
use JNI::{
    JNI_CreateJavaVM, JNI_TRUE, JNI_VERSION_21, JNIEnv, JavaVM, JavaVMInitArgs, JavaVMOption,
    jclass, jfieldID, jmethodID, jobject, jstring, jvalue,
};

impl JNIEnv {
    gen_jni_method!(NewStringUTF,
        jstring,
        utf: *const ::std::os::raw::c_char);
    gen_jni_method!(FindClass,
        jclass,
        name: *const ::std::os::raw::c_char);
    gen_jni_method!(GetMethodID,
        jmethodID,
        clazz: jclass,name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char);
    gen_jni_method!(GetStaticObjectField,
        jobject,
        clazz: jclass,
        fieldID: jfieldID);
    gen_jni_method!(GetStaticFieldID,
        jfieldID,
        clazz: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char);
    gen_jni_method!(CallVoidMethodA,
        (),
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue);
}

impl JavaVM {
    gen_jni_method!(DestroyJavaVM, i32);
}

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
        let system_class = (*jenv).FindClass(c!("java/lang/System"));
        let outField =
            (*jenv).GetStaticFieldID(system_class, c!("out"), c!("Ljava/io/PrintStream;"));
        let outObj = (*jenv).GetStaticObjectField(system_class, outField);
        let printStreamClass = (*jenv).FindClass(c!("java/io/PrintStream"));
        let printlnMethod =
            (*jenv).GetMethodID(printStreamClass, c!("println"), c!("(Ljava/lang/String;)V"));
        let msg = (*jenv).NewStringUTF(c!("Hello World"));
        let arg = [jvalue { l: msg as jobject }];
        (*jenv).CallVoidMethodA(outObj, printlnMethod, arg.as_ptr());
        (*jvm).DestroyJavaVM();
    }
}
