use PersonWrap::Person;

use JNIWrap::CreateJavaWrapper;
use JNIWrap::JNI::{JNI_VERSION_21, JavaVMInitArgs, JavaVMOption};
fn main() {
    let mut vmoptions = JavaVMOption::new(r"-Djava.class.path=PersonWrap/java");
    let vmargs = JavaVMInitArgs::new(JNI_VERSION_21, 1, &mut vmoptions, true);
    let (jvm, jenv) = CreateJavaWrapper(vmargs);
    let mut zhangsan = unsafe { Person::new(jenv, "zhangsan", 18) }.unwrap();
    unsafe {
        zhangsan.introduce().unwrap();
        (*jvm).DestroyJavaVM();
    }
    println!("Finish");
}
