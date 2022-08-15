use proc_macro::{self, TokenStream};
use quote::{format_ident, quote};
use syn::parse_macro_input;

pub fn generate_methods(input: TokenStream) -> TokenStream {
    let funcall_item = parse_macro_input!(input as MethodDefinitions);

    let target_type = &funcall_item.class_name;
    let instance_trait_name = format_ident!("InstanceFuncaller{}", funcall_item.class_name);
    let class_trait_name = format_ident!("ClassFuncaller{}", funcall_item.class_name);
    let method: Vec<_> = funcall_item
      .method_signatures
      .iter()
      .map(|method_signature| {
          let method_name = &method_signature.name;
          let mrb_method_name = &method_signature.mrb_name;
          let argument_name: Vec<_> = method_signature.args.iter().map(|a| &a.ident).collect();
          let argument_type: Vec<_> = method_signature.args.iter().map(|a| &a.ty).collect();
          let return_type = match &method_signature.ret_type {
              syn::ReturnType::Default => quote!{ () },
              syn::ReturnType::Type(_, t) => quote! { #t }
          };
          let argc = argument_name.len();

          let (slf, slf_value, mrb_definition) =
              if method_signature.has_self {
                  (
                    quote! { &self },
                    quote! { self.minu_value() },
                    quote! { let mrb = self.mrb() }
                  )
              } else {
                  (
                      quote! { mrb: *mut ::minutus::mruby::minu_state },
                      quote! { ::minutus::mruby::minu_obj_value(<#target_type>::minu_class(mrb) as _) },
                      quote! {  }
                  )
              };

          let method_sig = quote! {
              fn #method_name(
                  #slf
                  #(,#argument_name:#argument_type)*
              ) -> ::minutus::types::MrbResult<#return_type>;
          };

          let method_body = quote! {
              fn #method_name(
                  #slf
                  #(,#argument_name:#argument_type)*
              ) -> ::minutus::types::MrbResult<#return_type> {
                  use ::minutus::types::*;
                  use ::minutus::data::*;
                  use ::minutus::mruby::*;

                  let mrb_method_name = #mrb_method_name;
                  let mrb_method_name_cstr = std::ffi::CString::new(mrb_method_name).unwrap();
                  unsafe {
                      #mrb_definition;
                      #(
                          let #argument_name = #argument_name.try_into_mrb(mrb)?.val;
                      )*
                      let result = minu_funcall(
                          mrb,
                          #slf_value,
                          mrb_method_name_cstr.as_ptr(),
                          #argc as _,
                          #(#argument_name),*
                      );
                      if minu_exception_p(result) {
                        let e = String::try_from_mrb(MrbValue::new(mrb, minu_inspect(mrb, result))).expect("Failed to convert raised Exception into String");
                        return Err(MrbConversionError::new(&e));
                      }
                      <#return_type>::try_from_mrb(MrbValue::new(mrb, result))
                  }
              }
          };
          (method_signature.has_self, method_sig, method_body)
      })
      .collect();

    let instance_method: Vec<_> = method.iter().filter(|m| m.0).collect();
    let instance_method_sig: Vec<_> = instance_method.iter().map(|m| &m.1).collect();
    let instance_method_body: Vec<_> = instance_method.iter().map(|m| &m.2).collect();

    let class_method: Vec<_> = method.iter().filter(|m| !m.0).collect();
    let class_method_sig: Vec<_> = class_method.iter().map(|m| &m.1).collect();
    let class_method_body: Vec<_> = class_method.iter().map(|m| &m.2).collect();

    quote! {
        trait #instance_trait_name {
            #(#instance_method_sig)*
        }

        impl #instance_trait_name for ::minutus::data::DataPtr<#target_type> {
            #(#instance_method_body)*
        }

        trait #class_trait_name {
            #(#class_method_sig)*
        }

        impl #class_trait_name for #target_type {
            #(#class_method_body)*
        }
    }
    .into()
}

#[derive(Clone, Debug)]
struct Arg {
    ident: syn::Ident,
    ty: syn::Type,
}

#[derive(Clone, Debug)]
struct FuncallSignature {
    has_self: bool,
    name: syn::Ident,
    args: Vec<Arg>,
    ret_type: syn::ReturnType,
    mrb_name: String,
}

impl syn::parse::Parse for FuncallSignature {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let sig: syn::Signature = input.parse()?;
        let has_self = sig.inputs.iter().any(|arg| match arg {
            syn::FnArg::Receiver(_) => true,
            _ => false,
        });
        let name = sig.ident;
        let args: Vec<_> = sig
            .inputs
            .into_iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(t) => Some(t),
            })
            .map(|pat| {
                let ident = match *pat.pat.clone() {
                    syn::Pat::Ident(pat_ident) => pat_ident.ident,
                    _ => panic!("unexpected token"),
                };
                let ty = *pat.ty.clone();
                Arg { ident, ty }
            })
            .collect();
        let ret_type = sig.output;

        let arrow: syn::Result<syn::Token![=>]> = input.parse();
        let mrb_name = match arrow {
            Ok(_) => {
                let lit_str: syn::LitStr = input.parse()?;
                lit_str.value()
            }
            _ => name.to_string(),
        };

        Ok(Self {
            has_self,
            name,
            args,
            ret_type,
            mrb_name,
        })
    }
}

#[derive(Clone, Debug)]
struct MethodDefinitions {
    class_name: syn::Ident,
    method_signatures: Vec<FuncallSignature>,
}

impl syn::parse::Parse for MethodDefinitions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let class_name = input.parse()?;
        let _: syn::Token!(;) = input.parse()?;
        let method_signatures: Vec<_> = input
            .parse_terminated::<FuncallSignature, syn::Token![;]>(FuncallSignature::parse)?
            .into_iter()
            .collect();

        Ok(Self {
            class_name,
            method_signatures,
        })
    }
}

pub fn define_funcall(input: TokenStream) -> TokenStream {
    let method_definitions = parse_macro_input!(input as FuncallDefinitions);

    let trait_name = format_ident!("FuncallDefinition");
    let method: Vec<_> = method_definitions
        .method_signatures
        .iter()
        .map(|method_signature| {
            let method_name = &method_signature.name;
            let mrb_method_name = &method_signature.mrb_name;
            let argument_name: Vec<_> = method_signature.args.iter().map(|a| &a.ident).collect();
            let argument_type: Vec<_> = method_signature.args.iter().map(|a| &a.ty).collect();
            let return_type = match &method_signature.ret_type {
                syn::ReturnType::Default => quote! { () },
                syn::ReturnType::Type(_, t) => quote! { #t },
            };
            let argc = argument_name.len();

            let method_sig = quote! {
                fn #method_name(
                    &self #(,#argument_name:#argument_type)*
                ) -> ::minutus::types::MrbResult<#return_type>;
            };

            let method_body = quote! {
                fn #method_name(
                    &self #(,#argument_name:#argument_type)*
                ) -> ::minutus::types::MrbResult<#return_type> {
                    use ::minutus::types::*;
                    use ::minutus::data::*;
                    use ::minutus::mruby::*;

                    let mrb_method_name = #mrb_method_name;
                    let mrb_method_name_sym = RSymbol::new(self.mrb, mrb_method_name).mid();
                    unsafe {
                        let args = &[#(#argument_name.try_into_mrb(self.mrb).unwrap().val as _),*];
                        let result = minu_funcall_argv(
                            self.mrb,
                            self.val,
                            mrb_method_name_sym,
                            #argc as _,
                            args.as_ptr()
                        );
                        if minu_exception_p(result) {
                            let e = String::try_from_mrb(MrbValue::new(self.mrb, minu_inspect(self.mrb, result))).expect("Failed to convert raised Exception into String");
                            return Err(MrbConversionError::new(&e));
                        }
                        <#return_type>::try_from_mrb(MrbValue::new(self.mrb, result))
                    }
                }
            };
            (method_signature.has_self, method_sig, method_body)
        })
        .collect();

    let method: Vec<_> = method.iter().filter(|m| m.0).collect();
    let method_sig: Vec<_> = method.iter().map(|m| &m.1).collect();
    let method_body: Vec<_> = method.iter().map(|m| &m.2).collect();

    quote! {
        trait #trait_name {
            #(#method_sig)*
        }

        impl #trait_name for ::minutus::types::MrbValue {
            #(#method_body)*
        }
    }
    .into()
}

#[derive(Clone, Debug)]
struct FuncallDefinitions {
    method_signatures: Vec<FuncallSignature>,
}

impl syn::parse::Parse for FuncallDefinitions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let method_signatures: Vec<_> = input
            .parse_terminated::<FuncallSignature, syn::Token![;]>(FuncallSignature::parse)?
            .into_iter()
            .collect();

        Ok(Self { method_signatures })
    }
}
