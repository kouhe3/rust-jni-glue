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
        name: &str,
    ) -> Option<jclass> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .FindClass?(self, c!(name));
            if result.is_null() { None } else { Some(result) }
        }
    }
    pub fn NewStringUTF(
        &mut self,
        utf: &str,
    ) -> Option<jstring> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .NewStringUTF?(self, c!(utf));
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
        name: &str,
        sig: &str,
    ) -> Option<jmethodID> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .GetStaticMethodID?(self, clazz, c!(name), c!(sig));
            if result.is_null() { None } else { Some(result) }
        }
    }

    pub fn GetMethodID(
        &mut self,
        clazz: jclass,
        name: &str,
        sig: &str,
    ) -> Option<jmethodID> {
        unsafe {
            let result = self
                .functions
                .as_ref()?
                .GetMethodID?(self, clazz, c!(name), c!(sig));
            if result.is_null() { None } else { Some(result) }
        }
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
                    .FindClass(class_name)
                    .expect("Failed FindClass"),
            }
            .into()
        }
    }
    pub fn MethodID(
        &mut self,
        name: &str,
        sig: &str,
    ) -> Option<jmethodID> {
        unsafe {
            (***self).GetMethodID(self.clazz, name, sig)
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
        name: &str,
        sig: &str,
    ) -> Option<jmethodID> {
        unsafe {
            (***self).GetStaticMethodID(self.clazz, name, sig)
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
