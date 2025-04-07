// 引入必要的包
use proc_macro::TokenStream as Tok1;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Expr, Ident, LitStr, ParenthesizedGenericArguments, Token, parse_macro_input};
use syn_derive::Parse;

/// 解析函数结构的结构体
/// 用于解析宏输入的函数定义，包含函数名、参数和返回类型等信息
#[derive(Parse)]
struct FnStruct {
    // 函数名
    fn_name: Ident,
    // 分隔符
    _col: Token![:],
    // 函数参数和返回类型
    args: ParenthesizedGenericArguments,
}

impl FnStruct {
    /// 展开函数定义为实际的 Rust 代码
    ///
    /// # 返回值
    /// 展开后的 TokenStream
    pub fn expand(&self) -> TokenStream {
        // 获取函数名
        let name = &self.fn_name;
        // 获取函数返回类型
        let ret = &self.args.output;
        // 生成参数标识符的基础名称
        let ident = Ident::new("n", Span::call_site());
        // 迭代函数的输入参数类型
        let types = self.args.inputs.iter();
        // 生成参数标识符
        let idents = (0..types.len()).map(|i| format_ident!("{}{}", ident, i));
        // 克隆参数标识符，用于函数调用
        let idents_clone = idents.clone();
        // 使用 quote 宏生成实际的 Rust 代码
        quote! {
            pub unsafe fn #name(&mut self,#(#idents: #types),*) #ret{
                unsafe {
                    let r = (*self.functions)
                        .#name?(self,#(#idents_clone),*);
                    Some(r)
                }
            }
        }
    }
}

/// 处理 jni_method 宏的过程宏
///
/// # 参数
/// - `input`: 宏输入的 TokenStream
///
/// # 返回值
/// 处理后的 TokenStream
#[proc_macro]
pub fn jni_method(input: Tok1) -> Tok1 {
    // 解析输入为 FnStruct 结构体
    let x: FnStruct = parse_macro_input!(input as FnStruct);
    // 展开结构体为实际的 Rust 代码
    let x = x.expand();
    // 将展开后的代码转换为 TokenStream 并返回
    proc_macro::TokenStream::from(x)
}

/// 用于解析构造函数体输入的结构体
/// 包含 JNI 环境指针、参数、类名和描述符等信息
#[derive(Parse)]
struct ConstructorBodyInput {
    // JNI 环境指针
    jenv: Expr,
    // 分隔符
    _comma1: Token![,],
    // 参数
    args: Expr,
    // 分隔符
    _comma2: Token![,],
    // 类名
    class_name: LitStr,
    // 分隔符
    _comma3: Token![,],
    // 描述符
    descriptor: LitStr,
}

/// 用于解析实例方法体输入的结构体
/// 包含 JNI 环境指针、参数、对象实例、方法名和描述符等信息
#[derive(Parse)]
struct MethodBodyInput {
    // JNI 环境指针
    jenv: Expr,
    // 分隔符
    _comma1: Token![,],
    // 参数
    args: Expr,
    // 分隔符
    _comma2: Token![,],
    // 对象实例
    obj: Expr,
    // 分隔符
    _comma4: Token![,],
    // 方法名
    method_name: LitStr,
    // 分隔符
    _comma: Token![,],
    // 描述符
    descriptor: LitStr,
}

/// 用于解析静态方法体输入的结构体
/// 包含 JNI 环境指针、参数、类名、方法名和描述符等信息
#[derive(Parse)]
struct StaticMethodBodyInput {
    // JNI 环境指针
    jenv: Expr,
    // 分隔符
    _comma1: Token![,],
    // 参数
    args: Expr,
    // 分隔符
    _comma2: Token![,],
    // 类名
    class_name: LitStr,
    // 分隔符
    _comma3: Token![,],
    // 方法名
    method_name: LitStr,
    // 分隔符
    _comma4: Token![,],
    // 描述符
    descriptor: LitStr,
}

/// 处理 jni_constructor_body 宏的过程宏
/// 用于生成调用 JNI 构造函数的代码
///
/// # 参数
/// - `input`: 宏输入的 TokenStream
///
/// # 返回值
/// 处理后的 TokenStream
#[proc_macro]
pub fn jni_constructor_body(input: Tok1) -> Tok1 {
    // 解析输入为 ConstructorBodyInput 结构体
    let input = parse_macro_input!(input as ConstructorBodyInput);
    // 获取 JNI 环境指针
    let jenv = input.jenv;
    // 获取参数
    let args = input.args;
    // 获取类名
    let class_name_str = input.class_name;
    // 获取描述符
    let descriptor_str = input.descriptor;
    // 使用 quote 宏生成调用 JNI 构造函数的代码
    let x = quote! {
        unsafe{
            let jenv_ptr = #jenv;
            let args_ptr = #args;
            let class_name_cstr = ::std::ffi::CString::new(#class_name_str).expect("Invalid class name literal provided to jni_constructor_body");
            let class = (*jenv_ptr).FindClass(class_name_cstr.as_ptr())?;
            if class.is_null() {
                return None;
            }
            let constructor_name_cstr = ::std::ffi::CString::new("<init>").unwrap();
            let descriptor_cstr = ::std::ffi::CString::new(#descriptor_str).expect("Invalid descriptor literal provided to jni_constructor_body");
            let method_id = (*jenv_ptr).GetMethodID(class, constructor_name_cstr.as_ptr(), descriptor_cstr.as_ptr())?;
            if method_id.is_null() {
                return None;
            }
            let obj = (*jenv_ptr).NewObjectA(class, method_id, args_ptr)?;
            if obj.is_null() {
                return None;
            }
            Some(Self(obj))
        }
    };
    // 将生成的代码转换为 TokenStream 并返回
    Tok1::from(x)
}

/// 处理 jni_method_body 宏的过程宏
/// 用于生成调用 JNI 实例方法的代码
///
/// # 参数
/// - `input`: 宏输入的 TokenStream
///
/// # 返回值
/// 处理后的 TokenStream
#[proc_macro]
pub fn jni_method_body(input: Tok1) -> Tok1 {
    // 解析输入为 MethodBodyInput 结构体
    let input = parse_macro_input!(input as MethodBodyInput);
    // 获取对象实例
    let obj = input.obj;
    // 获取 JNI 环境指针
    let jenv = input.jenv;
    // 获取参数
    let args = input.args;
    // 获取方法名
    let method_name = input.method_name;
    // 获取描述符
    let descriptor = input.descriptor;
    // 根据描述符确定返回类型
    let return_type = return_type(descriptor.value());
    // 生成调用方法的标识符
    let identname = format_ident!("Call{}MethodA", return_type);
    // 使用 quote 宏生成调用 JNI 实例方法的代码
    let x = quote! {
        unsafe {
            let method_name_cstr = ::std::ffi::CString::new(#method_name).expect("Invalid method name string literal");
            let descriptor_cstr = ::std::ffi::CString::new(#descriptor).expect("Invalid descriptor string literal");
            let jenv_ptr = #jenv;
            let args_ptr = #args;
            let self_obj = #obj;
            let class = (*jenv_ptr).GetObjectClass(self_obj)?;
            if class.is_null() {
                return None;
            }
            let method_id = (*jenv_ptr).GetMethodID(class, method_name_cstr.as_ptr(), descriptor_cstr.as_ptr())?;
            if method_id.is_null() {
                return None;
            }
            let r = (*jenv_ptr).#identname(self_obj, method_id, args_ptr)?;
            Some(r)
        }
    };
    // 将生成的代码转换为 TokenStream 并返回
    Tok1::from(x)
}

/// 处理 jni_static_method_body 宏的过程宏
/// 用于生成调用 JNI 静态方法的代码
///
/// # 参数
/// - `input`: 宏输入的 TokenStream
///
/// # 返回值
/// 处理后的 TokenStream
#[proc_macro]
pub fn jni_static_method_body(input: Tok1) -> Tok1 {
    // 解析输入为 StaticMethodBodyInput 结构体
    let input = parse_macro_input!(input as StaticMethodBodyInput);
    // 获取 JNI 环境指针
    let jenv = input.jenv;
    // 获取参数
    let args = input.args;
    // 获取类名
    let class_name = input.class_name;
    // 获取方法名
    let method_name = input.method_name;
    // 获取描述符
    let descriptor = input.descriptor;
    // 根据描述符确定返回类型
    let return_type = return_type(descriptor.value());
    // 生成调用静态方法的标识符
    let identname = format_ident!("CallStatic{}MethodA", return_type);
    // 使用 quote 宏生成调用 JNI 静态方法的代码
    let x = quote! {
        unsafe {
            let class_name_cstr = ::std::ffi::CString::new(#class_name).expect("Invalid class name string literal");
            let method_name_cstr = ::std::ffi::CString::new(#method_name).expect("Invalid method name string literal");
            let descriptor_cstr = ::std::ffi::CString::new(#descriptor).expect("Invalid descriptor string literal");
            let jenv_ptr = #jenv;
            let args_ptr = #args;
            let class = (*jenv_ptr).FindClass(class_name_cstr.as_ptr())?;
            if class.is_null() {
                return None;
            }
            let method_id = (*jenv_ptr).GetStaticMethodID(class, method_name_cstr.as_ptr(), descriptor_cstr.as_ptr())?;
            if method_id.is_null() {
                return None;
            }
            let r = (*jenv_ptr).#identname(class, method_id, args_ptr)?;
            Some(r)
        }
    };
    // 将生成的代码转换为 TokenStream 并返回
    Tok1::from(x)
}

/// 根据描述符确定返回类型
///
/// # 参数
/// - `descriptor`: 方法描述符
///
/// # 返回值
/// 返回类型的字符串表示
fn return_type(descriptor: String) -> &'static str {
    // 获取描述符的最后一个字符
    match descriptor.chars().last() {
        // 如果是 'I'，表示返回类型为整数
        Some('I') => "Int",
        // 如果是 'V'，表示返回类型为 void
        Some('V') => "Void",
        // 如果是 ';'，表示返回类型为对象
        Some(';') => "Object",
        // 其他情况抛出异常
        _ => panic!("Invalid return type"),
    }
}
