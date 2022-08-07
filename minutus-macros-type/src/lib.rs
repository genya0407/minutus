fn define_tuple(n: usize) -> proc_macro2::TokenStream {
    let ty: Vec<_> = (1..=n)
        .into_iter()
        .map(|i| quote::format_ident!("T{}", i as usize))
        .collect();
    let index: Vec<_> = (0..(n as i64)).into_iter().collect();
    let field_access: Vec<proc_macro2::TokenStream> = (0..n)
        .into_iter()
        .map(|i| format!("self.{}", i).parse().unwrap())
        .collect();

    let q = quote::quote! {
        impl<#(#ty: FromMrb<#ty>),*> FromMrb<Self> for (#(#ty,)*) {
            fn from_mrb(mrb: *mut minu_state, value: &minu_value) -> Self {
                unsafe {
                    if minu_array_p(*value) {
                        let len = minu_rarray_len(*value) as usize;
                        if len == #n {
                            return (
                                #(
                                    #ty::from_mrb(mrb, &minu_ary_ref(*value, #index)),
                                )*
                            )
                        }
                    }

                    let tname = format!("Tuple(len = {})", #n);
                    crate::utils::raise_type_mismatch_argument_error(mrb, *value, &tname)
                }
            }
        }

        impl<#(#ty: IntoMrb),*> IntoMrb for (#(#ty,)*) {
            fn into_mrb(self, mrb: *mut minu_state) -> minu_value {
                unsafe {
                    let ary = minu_ary_new_capa(mrb, #n as _);
                    #(
                        minu_ary_push(mrb, ary, #field_access.into_mrb(mrb));
                    )*
                    return ary;
                }
            }
        }

    };
    q.into()
}

#[proc_macro]
pub fn define_tuples(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let definitions: Vec<_> = (1..=32).map(|i| define_tuple(i)).collect();

    let q = quote::quote! {
        #(
            #definitions
        )*
    };
    q.into()
}
