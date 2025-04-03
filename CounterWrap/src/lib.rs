macro_rules! c {
    ($s:expr) => {
        ::std::ffi::CString::new($s).unwrap().into_raw()
    };
}

use JNIWrap::JNI::{jobject, jvalue, JNIEnv,jint};

pub struct Counter{
    pub obj: jobject,
    pub env: *mut JNIEnv,
}
impl Counter{
    pub unsafe fn add(jenv: *mut JNIEnv, a: i32, b:i32) -> Option<i32>{
        unsafe{
            let class = (*jenv).FindClass(c!("Counter"))?;
            let method = (*jenv).GetStaticMethodID(class, c!("add"), c!("(II)I"))?;
            let jvalue_list = [jvalue::jint(a), jvalue::jint(b)];
            (*jenv).CallStaticIntMethodA(class, method, jvalue_list.as_ptr()).map(|x| x as i32)
            
        }
        
    }
    pub unsafe fn main(jenv: *mut JNIEnv, args: &[&str]) -> Option<()>{
        unsafe {
            let class = (*jenv).FindClass(c!("Counter"))?;
            let method = (*jenv).GetStaticMethodID(class, c!("main"), c!("([Ljava/lang/String;)V"))?;
            let jvalue_list: Vec<jvalue> = args.iter().map(|s| unsafe{jvalue::str(jenv, s)}).filter_map(|f| Some(f)).collect();
            (*jenv).CallStaticVoidMethodA(class, method, jvalue_list.as_ptr())

        }
        
    }
}