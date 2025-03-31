use PersonWrap::Person;
use CounterWrap::Counter;
use JNIWrap::CreateJavaWrapper;
use JNIWrap::JNI::{JNI_VERSION_21, JavaVMInitArgs, JavaVMOption};
fn main() -> std::io::Result<()> {
    let mut vmoptions = JavaVMOption::new(r"-Djava.class.path=CounterWrap/java;PersonWrap/java");
    let vmargs = JavaVMInitArgs::new(JNI_VERSION_21, 1, &mut vmoptions, true);
    let (jvm, jenv) = CreateJavaWrapper(vmargs);
    let mut zhangsan = unsafe { Person::new(jenv, "zhangsan", 18) }.unwrap();
    unsafe {
        Counter::main(jenv, &["x"]);
        println!("5+8={}", Counter::add(jenv, 5, 8).unwrap());
        zhangsan.introduce().unwrap();
        (*jvm).DestroyJavaVM();
    }
    println!("Finish");
    Ok(())
}
