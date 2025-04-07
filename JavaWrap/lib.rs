pub mod java{
    pub mod io{
        use super::lang::AutoCloseable;
        trait Serializable{} 
        trait Closeable:AutoCloseable {
            
        }
        
    }
    pub mod lang{
        pub trait AutoCloseable {
            
        }
        pub mod Object{
            use std::ops::Deref;

            use jnimacro::jni_method_body;
            use JNIWrap::JNI::{jobject, jvalue, JNIEnv};

            use super::Class::Class;
            pub struct Object(pub Class);

        }
        pub trait Comparable {
            
        }
        pub trait CharSequence {
            
        }
        pub trait Constable {
            
        }
        pub trait ConstantDesc {
            
        }
        pub mod ClassLoader{
            use JNIWrap::JNI::jobject;

            pub struct ClassLoader(pub jobject);
        }
        pub mod Class{
            use jnimacro::jni_constructor_body;
            use JNIWrap::JNI::{jobject, jvalue, JNIEnv};

           pub struct Class(pub jobject); 
           impl Class{
                pub fn new(jenv: *mut JNIEnv, loader: super::ClassLoader::ClassLoader, arrayComponentType: Class) -> Option<Self>{
                    let _args = [jvalue::obj(loader.0),jvalue::obj(arrayComponentType.0)];
                    let args = _args.as_ptr();
                    let r = jni_constructor_body!(jenv, args, "java/lang/Class","(Ljava/lang/String;)V")?;
                    Some(r)
                } 
           }
        }
        pub mod String{
            use jnimacro::{jni_constructor_body, jni_static_method_body};
            use JNIWrap::JNI::{jvalue,jobject,JNIEnv};
            pub struct String(pub jobject);
            impl String{
                pub fn new(jenv: *mut JNIEnv, args: *const jvalue) -> Option<Self>{
                    let r = jni_constructor_body!(jenv, args, "java/lang/String","(Ljava/lang/String;)V")?;
                    Some(r)
                } 
            }
        }
        pub mod reflect{
            use JNIWrap::JNI::jobject;

            pub trait Type{
                fn into_raw(self) -> jobject;
            }
            impl Type for super::Class::Class {
                fn into_raw(self) -> jobject {
                    self.0
                }
            }
        }
    }
}