use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, ParenthesizedGenericArguments, Token, parse_macro_input};
#[derive(syn_derive::Parse)]
struct FnStruct {
    fn_name: Ident,
    _col: Token![:],
    args: ParenthesizedGenericArguments,
}
impl FnStruct {
    pub fn expand(&self) -> TokenStream {
        let name = &self.fn_name;
        let ret = &self
            .args
            .output;
        let ident = Ident::new("n", Span::call_site().into());
        let types = self
            .args
            .inputs
            .iter();
        let idents = (0..types.len()).map(|i| format_ident!("{}{}", ident, i));
        let idents_clone = idents.clone();
        quote! {
            pub fn #name(&mut self,#(#idents: #types),*) #ret{
                unsafe {
                    let r = self.functions
                        .as_ref()?
                        .#name?(self,#(#idents_clone),*);
                    Some(r)
                }
            }
        }
    }
}
#[proc_macro]
pub fn jni_method(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let x: FnStruct = parse_macro_input!(input as FnStruct);
    let x = x.expand();
    proc_macro::TokenStream::from(x)
}
