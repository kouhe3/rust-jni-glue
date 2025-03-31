use PersonWrap::Person;

use JNIWrap::CreateJavaWrapper;
use JNIWrap::JNI::{JNI_VERSION_21, JavaVMInitArgs, JavaVMOption};
fn main() {
    let mut vmoptions = JavaVMOption::new(r"-Djava.class.path=PersonWrap/java");
    let vmargs = JavaVMInitArgs::new(JNI_VERSION_21, 1, &mut vmoptions, true);
    let (mut jvm, mut jenv) = CreateJavaWrapper(vmargs);
    let mut zhangsan = Person::new(jenv, "zhangsan", 18).unwrap();
    zhangsan.introduce(jenv).unwrap();
    unsafe {
        (*jvm).DestroyJavaVM();
    }
    println!("Finish");
}
