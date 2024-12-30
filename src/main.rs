#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(c_variadic)]
#[macro_use]
mod macros;
use std::{ffi::CString, os::raw::c_void, ptr::null_mut};
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
}

impl JavaVM {
    fn DestroyJavaVM(&mut self) -> Option<i32> {
        unsafe { self.functions.as_ref().unwrap().DestroyJavaVM.unwrap()(self).into() }
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
        let counter = (*jenv).FindClass(c!("Counter")).expect("Find Class err");
        let mid = (*jenv)
            .GetStaticMethodID(counter, c!("add"), c!("(II)I"))
            .expect("Find Method err");
        let arg: [jvalue; 2] = [jvalue { i: 100 }, jvalue { i: 5 }];
        let sum = (*jenv)
            .CallStaticIntMethodA(counter, mid, arg.as_ptr())
            .expect("Call method err");
        println!("sum is {}", sum);
        (*jvm).DestroyJavaVM().expect("Destroy JVM err");
    }
    Ok(())
}
