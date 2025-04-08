use JNIWrap::JNI::{JNIEnv, jarray, jclass, jobject, jvalue};
use jnimacro::{jni_method_body, jni_static_method_body};

pub struct Object(jobject);
pub struct Class(pub jobject);
impl Class {
    pub fn getDeclaredConstructor(&mut self, jenv: *mut JNIEnv, args: *const jvalue) -> Class {
        Self(jni_method_body!(
            jenv,
            args,
            self.0,
            "getDeclaredConstructor",
            "([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;"
        ))
    }
    pub fn newInstance(&mut self, jenv: *mut JNIEnv, args: *const jvalue) -> jobject {
        let r = jni_method_body!(jenv, args, self.0, "newInstance", "()Ljava/lang/Object;");
        if r.is_null() {
            panic!("Can not create instance");
        }
        r
    }
}
pub struct StackTraceElement(pub jarray);

pub struct Thread(jobject);
impl Thread {
    pub unsafe fn currentThread(jenv: *mut JNIEnv, args: *const jvalue) -> Self {
        Self(jni_static_method_body!(
            jenv,
            args,
            "java/lang/Thread",
            "currentThread",
            "()Ljava/lang/Thread;"
        ))
    }
    pub fn getStackTrace(self, jenv: *mut JNIEnv, args: *const jvalue) -> StackTraceElement {
        let stack = jni_method_body!(
            jenv,
            args,
            self.0,
            "getStackTrace",
            "()[Ljava/lang/StackTraceElement;"
        );
        StackTraceElement(stack as jarray)
    }
}
