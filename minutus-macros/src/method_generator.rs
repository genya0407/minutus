use crate::name_generator::*;
use convert_case::Case::*;
use convert_case::Casing;
use darling::ToTokens;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Signature;

pub struct ArgumentInfo {
    ident: syn::Ident,
    is_reference: bool,
    ty: syn::Type,
}

pub struct ClassMethodGenerator {
    pub sig: Signature,
}

impl AbstractMethodGenerator for ClassMethodGenerator {
    fn name_generator(&self) -> Box<dyn MethodNameGenerator> {
        Box::new(ClassMethodNameGenerator {
            original_name: self.sig.ident.clone(),
        })
    }
    fn function_call(&self) -> TokenStream {
        let original_function_name = &self.sig.ident;
        quote! {Self::#original_function_name}
    }
    fn minu_define_method_name(&self) -> Ident {
        format_ident!("minu_define_class_method")
    }
    fn sig(&self) -> &Signature {
        &self.sig
    }
}

pub struct InstanceMethodGenerator {
    pub sig: Signature,
}

impl AbstractMethodGenerator for InstanceMethodGenerator {
    fn name_generator(&self) -> Box<dyn MethodNameGenerator> {
        Box::new(InstanceMethodNameGenerator {
            original_name: self.sig.ident.clone(),
        })
    }
    fn function_call(&self) -> TokenStream {
        let original_function = &self.sig.ident;
        quote! { Self::try_from_mrb(MrbValue::new(mrb, mrb_self)).unwrap().#original_function}
    }
    fn minu_define_method_name(&self) -> Ident {
        format_ident!("minu_define_method")
    }
    fn sig(&self) -> &Signature {
        &self.sig
    }
}

pub trait AbstractMethodGenerator {
    fn name_generator(&self) -> Box<dyn MethodNameGenerator>;
    fn function_call(&self) -> TokenStream;
    fn minu_define_method_name(&self) -> Ident;
    fn sig(&self) -> &Signature;

    fn argument_info(&self) -> Vec<ArgumentInfo> {
        let mut argument_info = vec![];
        for input in self.sig().inputs.iter() {
            let pat_type = match input {
                syn::FnArg::Typed(pat_type) => pat_type,
                _ => continue,
            };
            let ident = if let syn::Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                pat_ident.ident
            } else {
                panic!("unexpected function definition")
            };

            let (ty, is_reference): (syn::Type, bool) = match *pat_type.ty.clone() {
                syn::Type::Reference(reference) => {
                    if reference.mutability.is_some() {
                        panic!("can't pass mutable reference!")
                    };
                    (*reference.elem, true)
                }
                t => (t, false),
            };

            argument_info.push(ArgumentInfo {
                ident,
                ty,
                is_reference,
            });
        }
        argument_info
    }

    fn generate_externed_function(&self) -> TokenStream {
        let externed_function_name = self.name_generator().externed_function_name();
        let function_call = self.function_call();

        let argument_info = self.argument_info();
        let rust_argument_name: Vec<_> = argument_info.iter().map(|ai| &ai.ident).collect();
        let argument_type: Vec<_> = argument_info.iter().map(|ai| &ai.ty).collect();
        let argument_type_name: Vec<String> = argument_type
            .iter()
            .map(|i| format!("{}", i.to_token_stream()))
            .collect();
        let type_alias: Vec<_> = rust_argument_name
            .iter()
            .map(|name| format_ident!("{}", format!("alias_{}", name).to_case(Camel)))
            .collect();
        let constructor_name: Vec<TokenStream> = argument_info
            .iter()
            .map(|ai| {
                let base = format!("alias_{}", ai.ident).to_case(Camel);
                if ai.is_reference {
                    ["&", &base].join("").parse().unwrap()
                } else {
                    base.parse().unwrap()
                }
            })
            .collect();
        let argument_fmt = argument_info
            .iter()
            .map(|_| "o")
            .collect::<Vec<_>>()
            .join("");
        let mrb_argument_name: Vec<TokenStream> = argument_info
            .iter()
            .map(|ai| format!("mrb_arg_{}", ai.ident).parse().unwrap())
            .collect();

        quote! {
            unsafe extern "C" fn #externed_function_name(
                mrb: *mut ::minutus::mruby::minu_state,
                mrb_self: ::minutus::mruby::minu_value,
            ) -> ::minutus::mruby::minu_value {
                use ::minutus::types::*;

                #(
                    let mut #mrb_argument_name: ::minutus::mruby::minu_value = std::mem::zeroed();
                )*
                ::minutus::mruby::minu_get_args(
                    mrb,
                    std::ffi::CString::new(#argument_fmt).unwrap().as_ptr(),
                    #(&mut #mrb_argument_name),*
                );
                #(
                    // In order to use types with generics
                    type #type_alias = #argument_type;
                )*
                #(
                    let #rust_argument_name = #constructor_name::try_from_mrb(MrbValue::new(mrb, #mrb_argument_name));
                    let mut #rust_argument_name: #constructor_name = match #rust_argument_name {
                        Ok(val) => val,
                        Err(error) => {
                            ::minutus::mruby::raise_type_mismatch_argument_error(
                                mrb,
                                #mrb_argument_name,
                                #argument_type_name.to_string(),
                                error.msg.clone()
                            )
                        }
                    };
                )*

                #function_call(#(#rust_argument_name),*).try_into_mrb(mrb).unwrap().val
            }
        }
    }

    fn generate_method_define_function(&self) -> TokenStream {
        let method_define_function_name = self.name_generator().method_define_function_name();
        let minu_define_method_name = self.minu_define_method_name();
        let original_function_name = format!("{}", &self.sig().ident);
        let externed_function_name = self.name_generator().externed_function_name();
        let argc = self.argument_info().len() as u32;

        quote! {
            fn #method_define_function_name(mrb: *mut ::minutus::mruby::minu_state, class: *mut ::minutus::mruby::RClass) {
                unsafe {
                    ::minutus::mruby::#minu_define_method_name(
                        mrb,
                        class,
                        std::ffi::CString::new(#original_function_name).unwrap().as_ptr(),
                        Some(Self::#externed_function_name),
                        ::minutus::mruby::minu_MRB_ARGS_ARG(#argc, 0) // TODO: kwargs and other arities
                    )
                }
            }
        }
    }
}
