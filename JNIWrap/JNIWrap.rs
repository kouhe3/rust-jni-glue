// 允许使用非蛇形命名法，因为 JNI 相关的函数和类型通常遵循特定的命名规范
#![allow(non_snake_case)]
// 引入 jnimacro 库中的 jni_method 过程宏，用于生成 JNI 方法调用的代码
use jnimacro::jni_method;
// 公开 JNI 模块，该模块包含 JNI 相关的类型和函数定义
pub mod JNI;

// 引入 JNI 模块中的各种类型和函数
use JNI::{
    JNI_CreateJavaVM, JNIEnv, JavaVM, JavaVMInitArgs, JavaVMOption, jclass, jfieldID, jint, jlong,
    jmethodID, jobject, jstring, jvalue,
};
// 引入标准库中的相关类型和函数
use std::{ffi::CString, os::raw::c_void, ptr::null_mut};

/// 创建 Java 虚拟机和 JNI 环境的包装函数
///
/// 该函数接受 Java 虚拟机的初始化参数，并尝试创建 Java 虚拟机和 JNI 环境。
///
/// # 参数
/// - `vm_args`: Java 虚拟机的初始化参数
///
/// # 返回值
/// 一个元组，包含指向 Java 虚拟机的指针和指向 JNI 环境的指针
pub fn CreateJavaWrapper(mut vm_args: JavaVMInitArgs) -> (*mut JavaVM, *mut JNIEnv) {
    // 初始化指向 Java 虚拟机的指针为空
    let mut pjvm = null_mut::<JavaVM>();
    // 初始化指向 JNI 环境的指针为空
    let mut pjenv = null_mut::<JNIEnv>();
    // 由于 JNI 函数是不安全的，需要使用 unsafe 块
    unsafe {
        // 调用 JNI_CreateJavaVM 函数创建 Java 虚拟机和 JNI 环境
        JNI_CreateJavaVM(
            &mut pjvm,
            &mut pjenv as *mut *mut JNIEnv as *mut *mut c_void,
            &mut vm_args as *mut JavaVMInitArgs as *mut c_void,
        );

        // 返回指向 Java 虚拟机和 JNI 环境的指针
        (pjvm, pjenv)
    }
}

/// JavaVMOption 结构体的实现
impl JavaVMOption {
    /// 创建一个新的 JavaVMOption 实例
    ///
    /// 该函数接受一个字符串参数，并将其转换为 C 字符串，用于设置 Java 虚拟机的选项。
    ///
    /// # 参数
    /// - `s`: 用于设置 Java 虚拟机选项的字符串
    ///
    /// # 返回值
    /// 一个新的 JavaVMOption 实例
    pub fn new(s: &str) -> Self {
        JavaVMOption {
            // 额外信息指针初始化为空
            extraInfo: std::ptr::null_mut(),
            // 将输入的字符串转换为 C 字符串，并获取其指针
            optionString: CString::new(s).unwrap().into_raw(),
        }
    }
}

/// JavaVMInitArgs 结构体的实现
impl JavaVMInitArgs {
    /// 创建一个新的 JavaVMInitArgs 实例
    ///
    /// 该函数接受 Java 虚拟机的版本、选项数量、选项数组和是否忽略未识别选项的标志，并返回一个新的 JavaVMInitArgs 实例。
    ///
    /// # 参数
    /// - `version`: Java 虚拟机的版本
    /// - `nOptions`: 选项的数量
    /// - `options`: 选项数组的可变引用
    /// - `ignoreUnrecognized`: 是否忽略未识别的选项
    ///
    /// # 返回值
    /// 一个新的 JavaVMInitArgs 实例
    pub fn new(
        version: u32,
        nOptions: i32,
        options: &mut JavaVMOption,
        ignoreUnrecognized: bool,
    ) -> Self {
        JavaVMInitArgs {
            // 将版本转换为 i32 类型
            version: version as i32,
            // 选项的数量
            nOptions,
            // 选项数组的引用
            options,
            // 将布尔值转换为 u8 类型
            ignoreUnrecognized: ignoreUnrecognized as u8,
        }
    }
}

/// JNIEnv 结构体的实现，定义 JNI 环境的方法
impl JNIEnv {
    jni_method!(NewObjectA: (jclass, jmethodID, *const jvalue) -> Option<jobject>  );
    jni_method!(FindClass: (*const ::std::os::raw::c_char) -> Option<jclass>);
    jni_method!(NewStringUTF: (*const ::std::os::raw::c_char) -> Option<jstring>);
    jni_method!(CallStaticVoidMethodA: (jclass, jmethodID, *const jvalue) -> Option<()>);
    jni_method!(CallStaticIntMethodA: (jclass, jmethodID, *const jvalue) -> Option<jint>);
    jni_method!(CallVoidMethodA: (jobject, jmethodID, *const jvalue) -> Option<()>);
    jni_method!(CallObjectMethodA: (jobject, jmethodID, *const jvalue) -> Option<jobject>);
    jni_method!(CallStaticObjectMethodA: (jclass, jmethodID, *const jvalue) -> Option<jobject>);
    jni_method!(GetStaticMethodID: (jclass, *const ::std::os::raw::c_char, *const ::std::os::raw::c_char) -> Option<jmethodID>);
    jni_method!(GetMethodID: (jclass, *const ::std::os::raw::c_char, *const ::std::os::raw::c_char) -> Option<jmethodID>);
    jni_method!(GetObjectField: (jobject, jfieldID) -> Option<jobject>);
    jni_method!(GetObjectClass: (jobject) -> Option<jclass>);
}

/// JavaVM 结构体的实现，定义 Java 虚拟机的方法
impl JavaVM {
    jni_method!(DestroyJavaVM: () -> Option<i32>);
}

/// jvalue 结构体的实现，定义创建不同类型 jvalue 的方法
impl jvalue {
    /// 创建一个包含 Java 字符串的 `jvalue`。
    ///
    /// # Safety
    /// - `pjenv` 必须是一个有效的 `JNIEnv` 指针。
    /// - `s` 必须是一个有效的 UTF-8 字符串。
    /// - 调用者必须确保 `pjenv` 指向的 JNI 环境是有效的，并且当前线程已附加到 JVM。
    /// - 返回的 `jstring` 是一个本地引用，调用者需要确保在调用后正确管理其生命周期。
    pub unsafe fn str(jenv: *mut JNIEnv, s: &str) -> jvalue {
        jvalue {
            l: unsafe {
                (*jenv)
                    .NewStringUTF(CString::new(s).unwrap().as_ptr())
                    .unwrap() as jobject
            },
        }
    }

    /// 创建一个包含 `jclass` 的 `jvalue`。
    /// # Safety
    /// - `pjenv` 必须是一个有效的 `JNIEnv` 指针。
    /// - `s` 必须是一个有效的 UTF-8 字符串。
    /// - 调用者必须确保 `pjenv` 指向的 JNI 环境是有效的，并且当前线程已附加到 JVM。
    /// - 返回的 `jclass` 是一个本地引用，调用者需要确保在调用后正确管理其生命周期。
    pub unsafe fn class(jenv: *mut JNIEnv, s: &str) -> jvalue {
        jvalue {
            l: unsafe {
                (*jenv)
                    .FindClass(CString::new(s).unwrap().as_ptr())
                   .unwrap() as jobject
            }
        }
    }

    /// 创建一个包含 `jint` 的 `jvalue`。
    pub fn jint(i: jint) -> jvalue {
        jvalue { i }
    }

    /// 创建一个包含 `jlong` 的 `jvalue`。
    pub fn jlong(j: jlong) -> jvalue {
        jvalue { j }
    }

    /// 创建一个空的 `jvalue`（表示 `null`）。
    pub fn null() -> jvalue {
        jvalue { l: null_mut() }
    }

    pub fn obj(obj: jobject) -> jvalue {
        jvalue { l: obj }
    }
}
