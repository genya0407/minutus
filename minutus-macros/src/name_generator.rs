use proc_macro2::Ident;
use quote::format_ident;

pub trait MethodNameGenerator {
    fn method_define_function_name(&self) -> Ident;
    fn externed_function_name(&self) -> Ident;
}

pub struct ClassMethodNameGenerator {
    pub original_name: Ident,
}

impl MethodNameGenerator for ClassMethodNameGenerator {
    fn method_define_function_name(&self) -> Ident {
        format_ident!(
            "__minutus_define_method_for_class_method_{}",
            self.original_name
        )
    }
    fn externed_function_name(&self) -> Ident {
        format_ident!(
            "__minutus_externed_function_for_class_method_{}",
            self.original_name
        )
    }
}

pub struct InstanceMethodNameGenerator {
    pub original_name: Ident,
}

impl MethodNameGenerator for InstanceMethodNameGenerator {
    fn method_define_function_name(&self) -> Ident {
        format_ident!(
            "__minutus_define_method_for_instance_method_{}",
            self.original_name
        )
    }
    fn externed_function_name(&self) -> Ident {
        format_ident!(
            "__minutus_externed_function_for_instance_method_{}",
            self.original_name
        )
    }
}
