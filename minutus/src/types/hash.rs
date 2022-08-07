use super::*;
use std::collections::HashMap;

impl<K: FromMrb<K> + Eq + std::hash::Hash, V: FromMrb<V>> FromMrb<Self> for HashMap<K, V> {
    fn from_mrb(mrb: *mut minu_state, hash: &minu_value) -> Self {
        unsafe {
            if minu_hash_p(*hash) {
                let keys = minu_hash_keys(mrb, *hash);
                let values = minu_hash_values(mrb, *hash);
                let len = minu_hash_size(mrb, *hash) as usize;
                let mut hashmap = HashMap::new();
                for i in 0..len {
                    let k = K::from_mrb(mrb, &minu_ary_ref(keys, i as _));
                    let v = V::from_mrb(mrb, &minu_ary_ref(values, i as _));
                    hashmap.insert(k, v);
                }
                hashmap
            } else {
                crate::utils::raise_type_mismatch_argument_error(mrb, *hash, "HashMap")
            }
        }
    }
}

impl<K: IntoMrb, V: IntoMrb> IntoMrb for HashMap<K, V> {
    fn into_mrb(self, mrb: *mut minu_state) -> minu_value {
        unsafe {
            let hash = minu_hash_new_capa(mrb, self.len() as _);
            for (k, v) in self.into_iter() {
                minu_hash_set(mrb, hash, k.into_mrb(mrb), v.into_mrb(mrb));
            }
            return hash;
        }
    }
}
