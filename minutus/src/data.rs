use crate::mruby::*;

// TODO: mrb_gc_register / mrb_gc_unregister を使ってちゃんとGCと向き合う
// new 関数を作って register, drop するときに unregister すれば大丈夫な気がする
pub struct DerefPtr<T: Sized>(*mut T);

impl<T> std::ops::Deref for DerefPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.0) }
    }
}

pub trait MrbData: Sized {
    fn from_mrb_data<'a>(mrb: *mut minu_state, value: &minu_value) -> DerefPtr<Self> {
        unsafe { DerefPtr(minu_data_get_ptr(mrb, *value, Self::minu_data_type()) as *mut Self) }
    }
    fn into_mrb_data(self, mrb: *mut minu_state) -> minu_value {
        let size = std::mem::size_of::<Self>();
        unsafe {
            let mem = minu_malloc(mrb, size as u64) as *mut Self;
            core::ptr::write(mem, self);
            let rdata = minu_data_object_alloc(
                mrb,
                Self::minu_class(mrb),
                mem as *mut _,
                Self::minu_data_type(),
            );
            minu_obj_value(rdata as _)
        }
    }
    fn minu_class(mrb: *mut minu_state) -> *mut RClass;
    fn minu_data_type() -> *const minu_data_type;
}
