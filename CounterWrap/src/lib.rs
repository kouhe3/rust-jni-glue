use JNIWrap::JNI::{JNIEnv, jint, jobject, jvalue};
use jnimacro::*;
pub struct Counter(jobject);
impl Counter {
    pub unsafe fn add(jenv: *mut JNIEnv, args: *const jvalue) -> jint {
        jni_static_method_body!(jenv, args, "Counter", "add", "(II)I")
    }
    pub unsafe fn main(jenv: *mut JNIEnv, args: *const jvalue) -> () {
        jni_static_method_body!(jenv, args, "Counter", "main", "([Ljava/lang/String;)V")
    }
}
