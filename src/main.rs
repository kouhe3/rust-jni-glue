use ByteBuddyWrap::dynamic::DynamicType::MethodDefinition::ImplementationDefinition;
use ByteBuddyWrap::implementation::FixedValue::FixedValue;
use ByteBuddyWrap::{ByteBuddy::ByteBuddy, dynamic::DynamicType::Builder};
use CounterWrap::Counter;
use JNIWrap::CreateJavaWrapper;
use JNIWrap::JNI::{JNI_VERSION_21, JavaVMInitArgs, JavaVMOption, jvalue};
use JavaWrap::java::lang::Class::Class;
use JavaWrap::java::lang::Object::Object;
use JavaWrap::java::lang::String::String;
use PersonWrap::Person;
fn main() -> std::io::Result<()> {
    let mut vmoptions = JavaVMOption::new(
        r"-Djava.class.path=CounterWrap/java;PersonWrap/java;C:\Users\X\Downloads\cp",
    );
    let vmargs = JavaVMInitArgs::new(JNI_VERSION_21, 1, &mut vmoptions, true);
    let (jvm, jenv) = CreateJavaWrapper(vmargs);

    unsafe {
        let mut bubby = ByteBuddy::new(jenv).unwrap();

        let mut builder = bubby
            .subclass(jenv, Class(jvalue::class(jenv, "java/lang/Object").l))
            .unwrap(); 

        let mut init = builder
            .defineMethod(
                jenv,
                String(jvalue::str(jenv, "sayhi").l),
                Class(jvalue::class(jenv, "java/lang/String").l),
                0x00000001,
            )
            .unwrap();
        let receivetypedef = init
            .intercept(
                jenv,
                FixedValue::value(jenv, Object(Class(jvalue::str(jenv, "sayhi").l))).unwrap(),
            )
            .unwrap();
            receivetypedef.make(jenv).unwrap();
        (*jvm).DestroyJavaVM();
    }

    println!("Finish");
    Ok(())
}
