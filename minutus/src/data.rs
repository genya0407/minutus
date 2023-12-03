//! Bindings between Rust's struct and mruby's [RData](https://mruby.org/docs/api/headers/mruby_2Fdata.h.html)
//!
//! See also [wrap](../attr.wrap.html)

use crate::mruby::*;
use crate::types::*;

/// Container type for MrbData
pub struct DataPtr<T: Sized> {
    rusty_value_ptr: *mut T,
    minu_value: minu_value,
    mrb: *mut minu_state,
}

impl<T: Sized> DataPtr<T> {
    pub fn minu_value(&self) -> minu_value {
        self.minu_value
    }

    pub fn mrb(&self) -> *mut minu_state {
        self.mrb
    }
}

impl<T> std::ops::Deref for DataPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.rusty_value_ptr) }
    }
}

impl<T: MrbData> TryFromMrb for DataPtr<T> {
    fn try_from_mrb(value: MrbValue) -> MrbResult<Self> {
        T::try_from_mrb_data(value)
    }
}

/// Trait that handles type-casting between Rust's data and mruby's [RData](https://mruby.org/docs/api/headers/mruby_2Fdata.h.html).
///
/// This trait is implemented by `minutus::wrap` macro.
pub trait MrbData: Sized {
    fn try_from_mrb_data<'a>(value: MrbValue) -> MrbResult<DataPtr<Self>> {
        unsafe {
            Ok(DataPtr {
                rusty_value_ptr: minu_data_get_ptr(value.mrb, value.val, Self::minu_data_type())
                    as *mut Self,
                minu_value: value.val,
                mrb: value.mrb,
            })
        }
    }
    fn try_into_mrb_data(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
        let size = std::mem::size_of::<Self>();
        unsafe {
            let mem = minu_malloc(mrb, size) as *mut Self;
            core::ptr::write(mem, self);
            let rdata = minu_data_object_alloc(
                mrb,
                Self::minu_class(mrb),
                mem as *mut _,
                Self::minu_data_type(),
            );
            Ok(MrbValue::new(mrb, minu_obj_value(rdata as _)))
        }
    }
    fn minu_class_name() -> String;
    fn minu_class(mrb: *mut minu_state) -> *mut RClass;
    fn minu_data_type() -> *const minu_data_type;
}
