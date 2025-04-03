macro_rules! c {
    ($s:expr) => {
        ::std::ffi::CString::new($s).unwrap().into_raw()
    };
}

use JNIWrap::JNI::{JNIEnv, jobject, jvalue};
use jnimacro::*;
pub struct Person(jobject);
impl Person {
    pub unsafe fn new(jenv: *mut JNIEnv, args: *const jvalue) -> Option<Self> {
        jni_constructor_body!(jenv, args, "Person", "(Ljava/lang/String;I)V")
    }
    pub unsafe fn introduce(&mut self, jenv: *mut JNIEnv, args: *const jvalue) -> Option<()> {
        jni_void_method_body!(self, jenv, args, "introduce", "()V")
    }
}
