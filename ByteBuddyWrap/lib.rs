#![allow(non_snake_case)]

pub mod ByteBuddy {
    use std::ptr::null_mut;

    use JNIWrap::JNI::{JNIEnv, jobject, jvalue};
    use jnimacro::{jni_constructor_body, jni_method_body};
    use JavaWrap::java::lang::Class::Class;

    pub struct ByteBuddy(jobject);
    impl ByteBuddy {
        pub unsafe fn new(jenv: *mut JNIEnv) -> Option<Self> {
            let _args = [jvalue::null()];
            let args = _args.as_ptr();
            jni_constructor_body!(jenv, args, "net/bytebuddy/ByteBuddy", "()V")
        }
        pub unsafe fn subclass(
            &mut self,
            jenv: *mut JNIEnv,
            superType: Class,
        ) -> Option<impl super::dynamic::Builder> {
            let _args = [jvalue::obj(superType.0)];
            let args = _args.as_ptr();
            let r = jni_method_body!(
                jenv,
                args,
                self.0,
                "subclass",
                "(Ljava/lang/Class;)Lnet/bytebuddy/dynamic/DynamicType$Builder;"
            )?;
            Some((r,))
        }
    }
}
pub mod dynamic;
pub mod implementation;