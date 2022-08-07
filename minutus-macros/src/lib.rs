use class_deriviation_generator::generate_class_initializer;
use method_generator::AbstractMethodGenerator;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse_macro_input;

mod class_deriviation_generator;
mod method_generator;
mod name_generator;

#[proc_macro_attribute]
pub fn wrap(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let class_initializer =
        proc_macro2::TokenStream::from(generate_class_initializer(attrs, input.clone()));

    let input = proc_macro2::TokenStream::from(input);

    let tokens = quote! {
        #[derive(minutus::MrbData)]
        #input

        #class_initializer
    };
    tokens.into()
}

#[proc_macro_derive(MrbData)]
pub fn derive_data(input: TokenStream) -> TokenStream {
    class_deriviation_generator::derive_data(input)
}

#[proc_macro_attribute]
pub fn class_method(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let original_function_definition = proc_macro2::TokenStream::from(input.clone());
    let signature = parse_macro_input!(input as syn::ItemFn).sig;
    let method_generator = method_generator::ClassMethodGenerator { sig: signature };
    let externed_function = method_generator.generate_externed_function();
    let method_define_function = method_generator.generate_method_define_function();

    let output = quote! {
        #original_function_definition
        #externed_function
        #method_define_function
    };
    output.into()
}

#[proc_macro_attribute]
pub fn method(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let original_function_definition = proc_macro2::TokenStream::from(input.clone());
    let signature = parse_macro_input!(input as syn::ItemFn).sig;
    let method_generator = method_generator::InstanceMethodGenerator { sig: signature };
    let externed_function = method_generator.generate_externed_function();
    let method_define_function = method_generator.generate_method_define_function();

    let output = quote! {
        #original_function_definition
        #externed_function
        #method_define_function
    };
    output.into()
}
