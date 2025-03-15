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

impl JNIEnv {
    fn NewObjectA(
        &mut self,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<jobject> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .NewObjectA?(self, clazz, methodID, args);
            if result.is_null() { None } else { Some(result) }
        }
    }
    fn FindClass(
        &mut self,
        name: *const ::std::os::raw::c_char,
    ) -> Option<jclass> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .FindClass?(self, name);
            if result.is_null() { None } else { Some(result) }
        }
    }
    fn NewStringUTF(
        &mut self,
        utf: *const ::std::os::raw::c_char,
    ) -> Option<jstring> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .NewStringUTF?(self, utf);
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
            self.functions
                .as_ref()?
                .CallStaticVoidMethodA?(self, cls, methodID, args);
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
            let result = self
                .functions
                .as_ref()?
                .CallStaticIntMethodA?(self, clazz, methodID, args);
            Some(result)
        }
    }

    fn CallVoidMethodA(
        &mut self,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<()> {
        unsafe {
            self.functions
                .as_ref()?
                .CallVoidMethodA?(self, obj, methodID, args);
            Some(())
        }
    }

    fn GetStaticMethodID(
        &mut self,
        clazz: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> Option<jmethodID> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .GetStaticMethodID?(self, clazz, name, sig);
            if result.is_null() { None } else { Some(result) }
        }
    }

    fn GetMethodID(
        &mut self,
        clazz: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> Option<jmethodID> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .GetMethodID?(self, clazz, name, sig);
            if result.is_null() { None } else { Some(result) }
        }
    }

    fn new_J_class(
        &mut self,
        name: &str,
    ) -> Option<J_class> {
        J_class {
            JNIEnv: self,
            clazz: self.FindClass(c!(name))?,
        }
        .into()
    }
}

impl JavaVM {
    fn DestroyJavaVM(&mut self) -> Option<i32> {
        unsafe {
            self.functions
                .as_ref()
                .unwrap()
                .DestroyJavaVM
                .unwrap()(self)
            .into()
        }
    }
}

struct J_class {
    JNIEnv: *mut JNIEnv,
    clazz: jclass,
}

impl J_class {
    fn new(
        jenv: *mut JNIEnv,
        class_name: &str,
    ) -> Option<Self> {
        unsafe {
            J_class {
                JNIEnv: jenv,
                clazz: (*jenv)
                    .FindClass(c!(class_name))
                    .expect("Failed FindClass"),
            }
            .into()
        }
    }
    fn constructor_methodid(
        &mut self,
        sig: &str,
    ) -> Option<jmethodID> {
        unsafe {
            let r = (***self).GetMethodID(self.clazz, c!("<init>"), c!(sig))?;
            if r.is_null() { None } else { Some(r) }
        }
    }
    fn MethodID(
        &mut self,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> Option<jmethodID> {
        unsafe {
            let r = (***self).GetMethodID(self.clazz, name, sig)?;
            if r.is_null() { None } else { Some(r) }
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
        unsafe {
            let r = (***self).GetStaticMethodID(self.clazz, name, sig)?;
            if r.is_null() { None } else { Some(r) }
        }
    }

    fn ObjectA(
        &mut self,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<jobject> {
        unsafe { (***self).NewObjectA(self.clazz, methodID, args) }
    }
}

struct Counter {
    J_class: J_class,
}

struct Person {
    J_class: J_class,
    jobject: jobject,
}

impl Deref for Person {
    type Target = J_class;
    fn deref(&self) -> &Self::Target {
        &self.J_class
    }
}

impl DerefMut for Person {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.J_class
    }
}

impl Person {
    fn new(
        jenv: *mut JNIEnv,
        name: jvalue,
        age: jvalue,
    ) -> Option<Self> {
        let mut person_clazz = J_class::new(jenv, "Person")?;
        let args = [name, age];
        let constructor = person_clazz.constructor_methodid("(Ljava/lang/String;I)V")?;
        let man = person_clazz.ObjectA(constructor, args.as_ptr())?;
        Person {
            J_class: person_clazz,
            jobject: man,
        }
        .into()
    }

    fn introduce(&mut self) -> Option<()> {
        let method = self.MethodID(c!("introduce"), c!("()V"))?;
        unsafe {
            let args: [jvalue; 0] = [];
            (****self).CallVoidMethodA(self.jobject, method, args.as_ptr())?;
            Some(())
        }
    }
}

impl DerefMut for Counter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.J_class
    }
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

impl Counter {
    fn add(
        jenv: *mut JNIEnv,
        a: jvalue,
        b: jvalue,
    ) -> Option<jint> {
        unsafe {
            let mut J_class = J_class::new(jenv, "Counter")?;
            let add = J_class.StaticMethodID(c!("add"), c!("(II)I"))?;
            let r = J_class.StaticIntMethodA(add, [a, b].as_ptr())?;
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
