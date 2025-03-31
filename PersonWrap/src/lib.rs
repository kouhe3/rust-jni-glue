macro_rules! c {
    ($s:expr) => {
        ::std::ffi::CString::new($s).unwrap().into_raw()
    };
}

use JNIWrap::JNI::{JNIEnv, jobject, jvalue};

pub struct Person {
    pub obj: jobject,
    pub env: *mut JNIEnv,
}
impl Person {
    pub unsafe fn new(jenv: *mut JNIEnv, name: &str, age: i32) -> Option<Self> {
        unsafe {
            let class = (*jenv).FindClass(c!("Person"))?;
            let constructor =
                (*jenv).GetMethodID(class, c!("<init>"), c!("(Ljava/lang/String;I)V"))?;
            let name = jvalue::str(jenv, name)?;
            let age = jvalue::jint(age)?;
            let args = [name, age];
            let p = (*jenv).NewObjectA(class, constructor, args.as_ptr())?;
            if p.is_null() {
                return None;
            }
            Some(Self { obj: p, env: jenv })
        }
    }
    pub unsafe fn introduce(&mut self) -> Option<()> {
        unsafe {
            let jenv = self.env;
            let class = (*jenv).GetObjectClass(self.obj)?;
            let method = (*jenv).GetMethodID(class, c!("introduce"), c!("()V"))?;
            (*jenv).CallVoidMethodA(self.obj, method, std::ptr::null())?;
            Some(())
        }
    }
}
