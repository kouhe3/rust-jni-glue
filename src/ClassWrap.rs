use crate::JNI::{
    JNI_FALSE, JNI_OK, JNI_TRUE, JNI_VERSION_21, JNIEnv, JavaVM, JavaVMInitArgs, JavaVMOption,
    jclass, jfieldID, jint, jmethodID, jobject, jstring, jvalue, va_list,
};
use std::ffi::CString;
use std::ops::{Deref, DerefMut};

use crate::JNIWrap::{J_class, J_methodid};
pub struct Counter {
    J_class: J_class,
}

pub struct Person {
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
    pub fn Person(
        jenv: *mut JNIEnv,
        name: &str,
        age: i32,
    ) -> Option<Self> {
        let mut this_class = J_class::new_FindClass(jenv, "Person")?;
        let name = jvalue::str(jenv, name)?;
        let age = jvalue::jint(age)?;
        let args = [name, age];
        let this_method = this_class.GetMethodID("<init>", "(Ljava/lang/String;I)V")?;
        let man = this_class.NewObjectA(this_method, args.as_ptr())?;
        Person {
            J_class: this_class,
            jobject: man,
        }
        .into()
    }
    pub fn introduce(&mut self) -> Option<()> {
        let method = self.GetMethodID("introduce", "()V")?;
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

impl Deref for Counter {
    type Target = J_class;
    fn deref(&self) -> &Self::Target {
        &self.J_class
    }
}

impl Counter {
    pub fn add(
        jenv: *mut JNIEnv,
        a: i32,
        b: i32,
    ) -> Option<jint> {
        let mut this_class = J_class::new_FindClass(jenv, "Counter")?;
        let mut this_method = J_methodid::new_GetStaticMethodID(&mut this_class, "add", "(II)I")?;
        let a = jvalue::jint(a)?;
        let b = jvalue::jint(b)?;
        let args = [a, b].as_ptr();
        let r = this_method.CallStaticIntMethodA(args)?;
        Some(r)
    }
    pub fn main(
        jenv: *mut JNIEnv,
        args: &[&str],
    ) -> Option<()> {
        let mut this_class = J_class::new_FindClass(jenv, "Counter")?;
        let mut this_method =
            J_methodid::new_GetStaticMethodID(&mut this_class, "main", "([Ljava/lang/String;)V")?;
        let mut v: Vec<jvalue> = Vec::new();
        for s in args {
            v.push(jvalue::str(jenv, s)?);
        }
        if v.is_empty() {
            v.push(jvalue::null()?);
        }
        this_method.CallStaticVoidMethodA(v.as_ptr());
        Some(())
    }
}
