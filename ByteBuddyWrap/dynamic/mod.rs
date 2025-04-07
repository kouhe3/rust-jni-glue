use JNIWrap::JNI::jvalue;

pub mod MethodDefinition {
    use JNIWrap::JNI::{JNIEnv, jobject, jvalue};
    use jnimacro::jni_method_body;

    use crate::implementation::Implementation;

    pub trait Initial: ParameterDefinition {}
    impl Initial for (jobject,) {}

    pub trait ParameterDefinition: ExceptionDefinition {}
    impl ParameterDefinition for (jobject,) {}

    pub trait ImplementationDefinition {
        fn intercept(
            &mut self,
            jenv: *mut JNIEnv,
            args: impl Implementation,
        ) -> Option<impl ReceiverTypeDefinition>;
    }
    impl ImplementationDefinition for (jobject,) {
        fn intercept(
            &mut self,
            jenv: *mut JNIEnv,
            args: impl Implementation,
        ) -> Option<impl ReceiverTypeDefinition> {
            let r = jni_method_body!(
                jenv,
                [jvalue::obj(args.into_raw())].as_ptr(),
                self.0,
                "intercept",
                "(Lnet/bytebuddy/implementation/Implementation;)Lnet/bytebuddy/dynamic/DynamicType$Builder$MethodDefinition$ReceiverTypeDefinition;"
            )?;
            Some((r,))
        }
    }

    pub trait ReceiverTypeDefinition {}
    impl ReceiverTypeDefinition for (jobject,) {}

    pub trait ExceptionDefinition: TypeVariableDefinition {}
    impl ExceptionDefinition for (jobject,) {}

    pub trait TypeVariableDefinition: ImplementationDefinition {}
    impl TypeVariableDefinition for (jobject,) {}
}
use JNIWrap::JNI::{JNIEnv, jint, jobject};
use jnimacro::*;
pub trait Builder {
    fn defineMethod(
        &mut self,
        jenv: *mut JNIEnv,
        var1: JavaWrap::java::lang::String::String,
        var2: impl JavaWrap::java::lang::reflect::Type,
        var3: jint,
    ) -> Option<impl MethodDefinition::Initial>;
    fn make(
        &mut self,
        jenv: *mut JNIEnv,
    ) -> Option<impl Unloaded>;
}
impl Builder for (jobject,) {
    fn defineMethod(
        &mut self,
        jenv: *mut JNIEnv,
        var1: JavaWrap::java::lang::String::String,
        var2: impl JavaWrap::java::lang::reflect::Type,
        var3: jint,
    ) -> Option<impl MethodDefinition::Initial> {
        let a = [
            jvalue::obj(var1.0),
            jvalue::obj(var2.into_raw()),
            jvalue::jint(var3),
        ];
        let args = a.as_ptr();
        let r = jni_method_body!(
            jenv,
            args,
            self.0,
            "defineMethod",
            "(Ljava/lang/String;Ljava/lang/reflect/Type;I)Lnet/bytebuddy/dynamic/DynamicType$Builder$MethodDefinition$ParameterDefinition$Initial;"
        )?;
        Some((r,))
    }
    fn make(
            &mut self,
            jenv: *mut JNIEnv,
        ) -> Option<impl Unloaded> {
        let r = jni_method_body!(
            jenv,
            [jvalue::null()].as_ptr(),
            self.0, 
            "make",
            "()Lnet/bytebuddy/dynamic/DynamicType$Unloaded;"
        )?;
        Some((r,))
    }
} 
pub trait ClassFileLocator {
    
}
pub trait DynamicType:ClassFileLocator {
    
}
pub trait Unloaded:DynamicType {
    
}