use CounterWrap::Counter;
use JNIWrap::CreateJavaWrapper;
use JNIWrap::JNI::{JNI_VERSION_21, JavaVMInitArgs, JavaVMOption, jvalue};
use PersonWrap::Person;
fn main() -> std::io::Result<()> {
    let mut vmoptions = JavaVMOption::new(r"-Djava.class.path=CounterWrap/java;PersonWrap/java");
    let vmargs = JavaVMInitArgs::new(JNI_VERSION_21, 1, &mut vmoptions, true);
    let (jvm, jenv) = CreateJavaWrapper(vmargs);
    let mut zhangsan = unsafe {
        Person::new(
            jenv,
            [jvalue::str(jenv, "zhangsan"), jvalue::jint(18)].as_ptr(),
        )
    }
    .unwrap();
    unsafe {
        Counter::main(jenv, [jvalue::str(jenv, "")].as_ptr());
        println!(
            "5+8={}",
            Counter::add(jenv, [jvalue::jint(5), jvalue::jint(8)].as_ptr()).unwrap()
        );
        zhangsan.introduce(jenv, std::ptr::null());
        (*jvm).DestroyJavaVM();
    }
    println!("Finish");
    Ok(())
}
