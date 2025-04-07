use JNIWrap::JNI::jobject;

pub mod FixedValue{

    use jnimacro::jni_static_method_body;
    use JNIWrap::JNI::{jobject, jvalue, JNIEnv};
    use JavaWrap::java::lang::{reflect::Type, Object::Object};

    use super::Implementation;
    
    pub struct FixedValue(jobject);
    impl FixedValue {
        pub unsafe fn value(jenv: *mut JNIEnv, value: Object) -> Option<impl AssignerConfigurable> {
            let _args = [jvalue::obj(value.0.0)];
            let args = _args.as_ptr();
            let r = jni_static_method_body!(jenv, args, "net/bytebuddy/implementation/FixedValue", "value", "(Ljava/lang/Object;)Lnet/bytebuddy/implementation/FixedValue$AssignerConfigurable;")?;
            Some((r,))
        }
    }

    pub trait AssignerConfigurable:Implementation {
        fn into_raw(self) -> jobject;
    }
    impl AssignerConfigurable for (jobject,) {
        fn into_raw(self) -> jobject {
           self.0
        }
    }
}

pub trait Implementation {
    fn into_raw(self) -> jobject;
        
    
}
impl Implementation for (jobject,) {
    fn into_raw(self) -> jobject {
        self.0 
    }
}