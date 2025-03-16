use crate::JNI::{
    JNI_FALSE, JNI_OK, JNI_TRUE, JNI_VERSION_21, JNIEnv, JavaVM, JavaVMInitArgs, JavaVMOption,
    jclass, jfieldID, jint, jmethodID, jobject, jstring, jvalue, va_list,
};
use std::ffi::CString;
use std::ops::{Deref, DerefMut};

use crate::JNIWrap::J_class;
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
    pub fn new(
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

    pub fn introduce(&mut self) -> Option<()> {
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
    pub fn add(
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
    pub fn main(
        jenv: *mut JNIEnv,
        args: *const jvalue,
    ) -> Option<()> {
        unsafe {
            let mut J_class = J_class::new(jenv, "Counter")?;
            let main = J_class.StaticMethodID(c!("main"), c!("([Ljava/lang/String;)V"))?;
            J_class.StaticVoidMethodA(main, args)?;
            Some(())
        }
    }
}
