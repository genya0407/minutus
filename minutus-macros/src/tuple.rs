pub fn define_tuple(n: usize) -> proc_macro2::TokenStream {
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
        impl<#(#ty: TryFromMrb),*> TryFromMrb for (#(#ty,)*) {
            fn try_from_mrb(value: MrbValue) -> MrbResult<Self> {
                unsafe {
                    if minu_array_p(value.val) {
                        let len = minu_rarray_len(value.val) as usize;
                        if len == #n {
                            return Ok((
                                #(
                                    #ty::try_from_mrb(MrbValue::new(value.mrb, minu_ary_ref(value.val, #index)))?,
                                )*
                            ))
                        }
                    }

                    Err(MrbConversionError::new(&format!("Tuple(len = {})", #n)))
                }
            }
        }

        impl<#(#ty: TryIntoMrb),*> TryIntoMrb for (#(#ty,)*) {
            fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
                unsafe {
                    let ary = minu_ary_new_capa(mrb, #n as _);
                    #(
                        minu_ary_push(mrb, ary, #field_access.try_into_mrb(mrb)?.val);
                    )*
                    return Ok(MrbValue::new(mrb, ary));
                }
            }
        }

    };
    q.into()
}
