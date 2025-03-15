#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(c_variadic)]
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
    let mut jvm = null_mut::<JavaVM>();

    let mut jenv = null_mut::<JNIEnv>();

    unsafe {
        JNI_CreateJavaVM(
            &mut jvm,
            &mut jenv as *mut *mut JNIEnv as *mut *mut c_void,
            &mut vm_args as *mut JavaVMInitArgs as *mut c_void,
        );

        let sum = Counter::add(jenv, jvalue { i: 1 }, jvalue { i: 2 }).unwrap();

        println!("sum is {}", sum);

        let name = (*jenv)
            .NewStringUTF(c!("zhangsan"))
            .unwrap();
        let age: jint = 18;
        let mut zhangsan = Person::new(jenv, jvalue { l: name as jobject }, jvalue { i: age })
            .expect("Can not create person");
        zhangsan
            .introduce()
            .expect("introduce err");

        (*jvm)
            .DestroyJavaVM()
            .expect("Destroy JVM err");
    }
    Ok(())
}
