use convert_case::Case::*;
use convert_case::Casing;
use darling::FromMeta;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

pub fn derive_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let class_name = format!("{}", ident);
    let data_type_name = format!("__minutus_{}_data\n", ident);
    let data_type_ident = format_ident!(
        "{}",
        format!("__minutus_data_type_{}", ident).to_case(Camel)
    );
    let dfree_ident = format_ident!("{}", format!("__minutus_{}_dfree", ident).to_case(Snake));

    let output = quote! {
        const #data_type_ident: ::minutus::mruby::minu_data_type = ::minutus::mruby::minu_data_type {
            struct_name: #data_type_name.as_ptr() as *const _,
            dfree: Some(#dfree_ident),
        };
        unsafe extern "C" fn #dfree_ident(
            mrb: *mut ::minutus::mruby::minu_state,
            ptr: *mut ::std::os::raw::c_void,
        ) {
            {
                let _: #ident = core::ptr::read(ptr as *const _);
            }
            ::minutus::mruby::minu_free(mrb, ptr);
        }

        impl ::minutus::data::MrbData for #ident {
            fn minu_class(mrb: *mut ::minutus::mruby::minu_state) -> *mut ::minutus::mruby::RClass {
                unsafe {
                    let class = ::minutus::mruby::minu_define_class(
                        mrb,
                        std::ffi::CString::new(#class_name).unwrap().as_ptr(),
                        (*mrb).object_class
                    );
                    ::minutus::mruby::minu_set_vtype_as_data(class);
                    return class;
                }
            }

            fn minu_data_type() -> *const ::minutus::mruby::minu_data_type {
                &#data_type_ident
            }
        }

        impl ::minutus::types::FromMrb<::minutus::data::DataPtr<#ident>> for #ident {
            fn from_mrb<'a>(mrb: *mut ::minutus::mruby::minu_state, value: &::minutus::mruby::minu_value) -> ::minutus::data::DataPtr<#ident> {
                use minutus::data::MrbData;

                #ident::from_mrb_data(mrb, value)
            }
        }

        impl ::minutus::types::IntoMrb for #ident {
            fn into_mrb(self, mrb: *mut ::minutus::mruby::minu_state) -> ::minutus::mruby::minu_value {
                use minutus::data::MrbData;

                #ident::into_mrb_data(self, mrb)
            }
        }
    };
    output.into()
}

#[derive(Default, FromMeta)]
struct WrapAttributes {
    #[darling(multiple)]
    class_method: Vec<String>,
    #[darling(multiple)]
    method: Vec<String>,
}

pub fn generate_class_initializer(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use crate::name_generator::*;

    let attr_args = parse_macro_input!(attr as syn::AttributeArgs);
    let wrap_attributes = WrapAttributes::from_list(&attr_args).unwrap();
    let class_method_initializer_ident: Vec<_> = wrap_attributes
        .class_method
        .iter()
        .map(|name| {
            ClassMethodNameGenerator {
                original_name: quote::format_ident!("{}", name),
            }
            .method_define_function_name()
        })
        .collect();
    let method_initializer_ident: Vec<_> = wrap_attributes
        .method
        .iter()
        .map(|name| {
            InstanceMethodNameGenerator {
                original_name: quote::format_ident!("{}", name),
            }
            .method_define_function_name()
        })
        .collect();
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    quote! {
      impl #ident {
        pub fn define_class_on_mrb(mrb: *mut ::minutus::mruby::minu_state) {
          use ::minutus::data::MrbData;

          unsafe {
              let class = Self::minu_class(mrb);
              #(
                  Self::#class_method_initializer_ident(mrb, class);
              )*
              #(
                  Self::#method_initializer_ident(mrb, class);
              )*
          }
        }
      }
    }
    .into()
}
