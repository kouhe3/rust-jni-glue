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
    JNI_CreateJavaVM, JNI_OK, JNI_TRUE, JNI_VERSION_21, JNIEnv, JavaVM, JavaVMInitArgs,
    JavaVMOption, jclass, jfieldID, jint, jmethodID, jobject, jstring, jvalue, va_list,
};

impl JNIEnv {
    fn FindClass(&mut self, name: *const ::std::os::raw::c_char) -> Option<jclass> {
        unsafe {
            let result = self.functions.as_ref()?.FindClass?(self, name);
            if result.is_null() { None } else { Some(result) }
        }
    }
    fn NewStringUTF(&mut self, utf: *const ::std::os::raw::c_char) -> Option<jstring> {
        unsafe {
            let result = self.functions.as_ref()?.NewStringUTF?(self, utf);
            if result.is_null() { None } else { Some(result) }
        }
    }
    fn CallStaticVoidMethodA(
        &mut self,
        cls: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<()> {
        unsafe {
            self.functions.as_ref()?.CallStaticVoidMethodA?(self, cls, methodID, args);
            Some(())
        }
    }

    fn CallStaticIntMethodA(
        &mut self,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<jint> {
        unsafe {
            let result =
                self.functions.as_ref()?.CallStaticIntMethodA?(self, clazz, methodID, args);
            Some(result)
        }
    }

    fn GetStaticMethodID(
        &mut self,
        clazz: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> Option<jmethodID> {
        unsafe {
            let result = self.functions.as_ref()?.GetStaticMethodID?(self, clazz, name, sig);
            if result.is_null() { None } else { Some(result) }
        }
    }

    fn new_J_class(&mut self, name: &str) -> Option<J_class> {
        J_class {
            JNIEnv: self,
            clazz: self.FindClass(c!(name))?,
        }
        .into()
    }
}

impl JavaVM {
    fn DestroyJavaVM(&mut self) -> Option<i32> {
        unsafe { self.functions.as_ref().unwrap().DestroyJavaVM.unwrap()(self).into() }
    }
}

struct J_class {
    JNIEnv: *mut JNIEnv,
    clazz: jclass,
}

impl J_class {
    fn new(jenv: *mut JNIEnv, class_name: &str) -> Option<Self> {
        unsafe {
            J_class {
                JNIEnv: jenv,
                clazz: (*jenv).FindClass(c!(class_name))?,
            }
            .into()
        }
    }
    unsafe fn StaticIntMethodA(
        &mut self,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<jint> {
        unsafe { (***self).CallStaticIntMethodA(self.clazz, methodID, args) }
    }
    unsafe fn StaticMethodID(
        &mut self,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> Option<jmethodID> {
        unsafe { (***self).GetStaticMethodID(self.clazz, name, sig) }
    }
}

struct Counter {
    J_class: J_class,
}

impl Deref for J_class {
    type Target = *mut JNIEnv;
    fn deref(&self) -> &Self::Target {
        &self.JNIEnv
    }
}
impl DerefMut for J_class {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.JNIEnv
    }
}

impl Deref for Counter {
    type Target = J_class;
    fn deref(&self) -> &Self::Target {
        &self.J_class
    }
}

impl DerefMut for Counter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.J_class
    }
}

impl Counter {
    fn new(jenv: *mut JNIEnv) -> Option<Self> {
        let J_class = J_class::new(jenv, "Counter")?;
        Counter { J_class }.into()
    }

    fn add(&mut self, a: jvalue, b: jvalue) -> Option<jint> {
        unsafe {
            let add = self.StaticMethodID(c!("add"), c!("(II)I"))?;
            let r = self.StaticIntMethodA(add, [a,b].as_ptr())?;
            Some(r)
        }
    }
}

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
        let mut counter = Counter::new(jenv).unwrap();
        let sum = counter.add(5, 10).unwrap();

        println!("sum is {}", sum);

        (*jvm).DestroyJavaVM().expect("Destroy JVM err");
    }
    Ok(())
}
