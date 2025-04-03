use proc_macro::TokenStream as Tok1;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Expr, Ident, LitStr, ParenthesizedGenericArguments, Token, parse_macro_input};
use syn_derive::Parse;

#[derive(Parse)]
struct FnStruct {
    fn_name: Ident,
    _col: Token![:],
    args: ParenthesizedGenericArguments,
}
impl FnStruct {
    pub fn expand(&self) -> TokenStream {
        let name = &self.fn_name;
        let ret = &self.args.output;
        let ident = Ident::new("n", Span::call_site());
        let types = self.args.inputs.iter();
        let idents = (0..types.len()).map(|i| format_ident!("{}{}", ident, i));
        let idents_clone = idents.clone();
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
#[proc_macro]
pub fn jni_method(input: Tok1) -> Tok1 {
    let x: FnStruct = parse_macro_input!(input as FnStruct);
    let x = x.expand();
    proc_macro::TokenStream::from(x)
}
#[derive(Parse)]
struct ConstructorBodyInput {
    jenv: Expr,
    _comma1: Token![,],
    args: Expr,
    _comma2: Token![,],
    class_name: LitStr,
    _comma3: Token![,],
    descriptor: LitStr,
}

#[derive(Parse)]
struct VoidMethodBodyInput {
    self_expr: Expr,
    _comma1: Token![,],
    jenv: Expr,
    _comma2: Token![,],
    args: Expr,
    _comma3: Token![,],
    method_name: LitStr,
    _comma4: Token![,],
    descriptor: LitStr,
}

#[proc_macro]
pub fn jni_constructor_body(input: Tok1) -> Tok1 {
    let input = parse_macro_input!(input as ConstructorBodyInput);
    let jenv = input.jenv;
    let args = input.args;
    let class_name_str = input.class_name;
    let descriptor_str = input.descriptor;
    let x = quote! {
        unsafe{
            let jenv_ptr = #jenv;
            let args_ptr = #args;
            let class_name_cstr = ::std::ffi::CString::new(#class_name_str).expect("Invalid class name literal provided to jni_constructor_body");
            let class = (*jenv_ptr).FindClass(class_name_cstr.as_ptr())?;
            let constructor_name_cstr = ::std::ffi::CString::new("<init>").unwrap();
            let descriptor_cstr = ::std::ffi::CString::new(#descriptor_str).expect("Invalid descriptor literal provided to jni_constructor_body");
            let method_id = (*jenv_ptr).GetMethodID(class, constructor_name_cstr.as_ptr(), descriptor_cstr.as_ptr())?;
            let obj = (*jenv_ptr).NewObjectA(class, method_id, args_ptr)?;
            Some(Self(obj))
        }
    };
    Tok1::from(x)
}

#[proc_macro]
pub fn jni_void_method_body(input: Tok1) -> Tok1 {
    let input = parse_macro_input!(input as VoidMethodBodyInput);
    let self_expr = input.self_expr;
    let jenv = input.jenv;
    let args = input.args;
    let method_name = input.method_name;
    let descriptor = input.descriptor;
    let x = quote! {
        unsafe {
            let method_name_cstr = ::std::ffi::CString::new(#method_name).expect("Invalid method name string literal");
            let descriptor_cstr = ::std::ffi::CString::new(#descriptor).expect("Invalid descriptor string literal");
            let jenv_ptr = #jenv;
            let args_ptr = #args;
            let self_obj = (#self_expr).0;
            let class = (*jenv_ptr).GetObjectClass(self_obj)?;
            let method_id = (*jenv_ptr).GetMethodID(class, method_name_cstr.as_ptr(), descriptor_cstr.as_ptr())?;
            (*jenv_ptr).CallVoidMethodA(self_obj, method_id, args_ptr)?;
            Some(())
        }
    };
    Tok1::from(x)
}
