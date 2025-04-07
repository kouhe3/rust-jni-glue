use JNIWrap::JNI::{JNIEnv, jobject, jvalue};
use jnimacro::*;
pub struct Person(jobject);
impl Person {
    pub unsafe fn new(jenv: *mut JNIEnv, args: *const jvalue) -> Self {
        jni_constructor_body!(jenv, args, "Person", "(Ljava/lang/String;I)V")
    }
    pub unsafe fn introduce(&mut self, jenv: *mut JNIEnv, args: *const jvalue) -> () {
        jni_method_body!(jenv, args, self.0, "introduce", "()V")
    }
}
