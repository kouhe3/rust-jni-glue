use crate::JNI::{
    JNI_CreateJavaVM, JNI_FALSE, JNI_OK, JNI_TRUE, JNI_VERSION_21, JNIEnv, JavaVM, JavaVMInitArgs,
    JavaVMOption, jclass, jfieldID, jint, jmethodID, jobject, jstring, jvalue, va_list,
};
use std::ffi::CString;
impl JNIEnv {
    pub fn NewObjectA(
        &mut self,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<jobject> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .NewObjectA?(self, clazz, methodID, args);
            if result.is_null() { None } else { Some(result) }
        }
    }
    pub fn FindClass(
        &mut self,
        name: *const ::std::os::raw::c_char,
    ) -> Option<jclass> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .FindClass?(self, name);
            if result.is_null() { None } else { Some(result) }
        }
    }
    pub fn NewStringUTF(
        &mut self,
        utf: *const ::std::os::raw::c_char,
    ) -> Option<jstring> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .NewStringUTF?(self, utf);
            if result.is_null() { None } else { Some(result) }
        }
    }
    pub fn CallStaticVoidMethodA(
        &mut self,
        cls: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<()> {
        unsafe {
            self.functions
                .as_ref()?
                .CallStaticVoidMethodA?(self, cls, methodID, args);
            Some(())
        }
    }

    pub fn CallStaticIntMethodA(
        &mut self,
        clazz: jclass,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<jint> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .CallStaticIntMethodA?(self, clazz, methodID, args);
            Some(result)
        }
    }

    pub fn CallVoidMethodA(
        &mut self,
        obj: jobject,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<()> {
        unsafe {
            self.functions
                .as_ref()?
                .CallVoidMethodA?(self, obj, methodID, args);
            Some(())
        }
    }

    pub fn GetStaticMethodID(
        &mut self,
        clazz: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> Option<jmethodID> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .GetStaticMethodID?(self, clazz, name, sig);
            if result.is_null() { None } else { Some(result) }
        }
    }

    pub fn GetMethodID(
        &mut self,
        clazz: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> Option<jmethodID> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .GetMethodID?(self, clazz, name, sig);
            if result.is_null() { None } else { Some(result) }
        }
    }

    pub fn new_J_class(
        &mut self,
        name: &str,
    ) -> Option<J_class> {
        J_class {
            JNIEnv: self,
            clazz: self.FindClass(c!(name))?,
        }
        .into()
    }
}

impl JavaVM {
    pub fn DestroyJavaVM(&mut self) -> Option<i32> {
        unsafe {
            self.functions
                .as_ref()
                .unwrap()
                .DestroyJavaVM
                .unwrap()(self)
            .into()
        }
    }
}

pub struct J_class {
    pub JNIEnv: *mut JNIEnv,
    clazz: jclass,
}

impl J_class {
    pub fn new(
        jenv: *mut JNIEnv,
        class_name: &str,
    ) -> Option<Self> {
        unsafe {
            J_class {
                JNIEnv: jenv,
                clazz: (*jenv)
                    .FindClass(c!(class_name))
                    .expect("Failed FindClass"),
            }
            .into()
        }
    }
    pub fn MethodID(
        &mut self,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> Option<jmethodID> {
        unsafe {
            let r = (***self).GetMethodID(self.clazz, name, sig)?;
            if r.is_null() { None } else { Some(r) }
        }
    }
    pub unsafe fn StaticIntMethodA(
        &mut self,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<jint> {
        unsafe { (***self).CallStaticIntMethodA(self.clazz, methodID, args) }
    }
    pub unsafe fn StaticVoidMethodA(
        &mut self,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<()> {
        unsafe { (***self).CallStaticVoidMethodA(self.clazz, methodID, args) }
    }
    pub unsafe fn StaticMethodID(
        &mut self,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> Option<jmethodID> {
        unsafe {
            let r = (***self).GetStaticMethodID(self.clazz, name, sig)?;
            if r.is_null() { None } else { Some(r) }
        }
    }

    pub fn ObjectA(
        &mut self,
        methodID: jmethodID,
        args: *const jvalue,
    ) -> Option<jobject> {
        unsafe { (***self).NewObjectA(self.clazz, methodID, args) }
    }
}
