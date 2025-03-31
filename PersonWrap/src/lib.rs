macro_rules! c {
    ($s:expr) => {
        ::std::ffi::CString::new($s).unwrap().into_raw()
    };
}

use JNIWrap::JNI::{JNIEnv, jobject, jvalue};

pub struct Person(jobject);
impl Person {
    pub fn new(jenv: *mut JNIEnv, name: &str, age: i32) -> Option<Self> {
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
            Some(Self(p))
        }
    }
    pub fn introduce(&mut self, jenv: *mut JNIEnv) -> Option<()> {
        unsafe {
            let class = (*jenv).GetObjectClass(self.0)?;
            let obj = self.0;
            let method = (*jenv).GetMethodID(class, c!("introduce"), c!("()V"))?;
            (*jenv).CallVoidMethodA(obj, method, std::ptr::null())?;
            Some(())
        }
    }
}
