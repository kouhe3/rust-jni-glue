use CounterWrap::Counter;
use JNIWrap::CreateJavaWrapper;
use JNIWrap::JNI::{
    JNI_VERSION_21, JNIEnv, JavaVMInitArgs, JavaVMOption, jclass, jobject, jobjectArray, jsize,
    jstring, jvalue,
};
use PersonWrap::Person;

use jnimacro::{jni_constructor_body, jni_method, jni_method_body};
use std::ffi::{CStr, CString};

pub struct HelloClass(pub jobject);
fn main() -> std::io::Result<()> {
    let mut vmoptions = JavaVMOption::new(
        r"-Djava.class.path=CounterWrap/java;PersonWrap/java;javassistWrap/java/javassist.jar",
    );
    let vmargs = JavaVMInitArgs::new(JNI_VERSION_21, 1, &mut vmoptions, true);
    let (jvm, jenv) = CreateJavaWrapper(vmargs);

    unsafe {
        test_counter(jenv);
        test_person(jenv);
        (*jvm).DestroyJavaVM();
    }
    println!("Finish");
    Ok(())
}

fn test_person(jenv: *mut JNIEnv) {
    let mut zhangsan = unsafe {
        Person::new(
            jenv,
            [jvalue::str(jenv, "zhangsan"), jvalue::jint(18)].as_ptr(),
        )
    };
    unsafe {
        zhangsan.introduce(jenv, std::ptr::null());
    }
}

fn test_counter(jenv: *mut JNIEnv) {
    unsafe {
        Counter::main(jenv, [jvalue::str(jenv, "")].as_ptr());
        println!(
            "5+8={}",
            Counter::add(jenv, [jvalue::jint(5), jvalue::jint(8)].as_ptr())
        );
    }
}
