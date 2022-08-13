use super::*;
use std::collections::HashMap;

impl<K: TryFromMrb + Eq + std::hash::Hash, V: TryFromMrb> TryFromMrb for HashMap<K, V> {
    fn try_from_mrb(value: MrbValue) -> MrbResult<Self> {
        unsafe {
            if minu_hash_p(value.val) {
                let keys = minu_hash_keys(value.mrb, value.val);
                let values = minu_hash_values(value.mrb, value.val);
                let len = minu_hash_size(value.mrb, value.val) as usize;
                let mut hashmap = HashMap::new();
                for i in 0..len {
                    let k = K::try_from_mrb(MrbValue::new(value.mrb, minu_ary_ref(keys, i as _)))?;
                    let v =
                        V::try_from_mrb(MrbValue::new(value.mrb, minu_ary_ref(values, i as _)))?;
                    hashmap.insert(k, v);
                }
                Ok(hashmap)
            } else {
                Err(MrbConversionError::new("Hash"))
            }
        }
    }
}

impl<K: TryIntoMrb, V: TryIntoMrb> TryIntoMrb for HashMap<K, V> {
    fn try_into_mrb(self, mrb: *mut minu_state) -> MrbResult<MrbValue> {
        unsafe {
            let hash = minu_hash_new_capa(mrb, self.len() as _);
            for (k, v) in self.into_iter() {
                minu_hash_set(
                    mrb,
                    hash,
                    k.try_into_mrb(mrb)?.val,
                    v.try_into_mrb(mrb)?.val,
                );
            }
            return Ok(MrbValue::new(mrb, hash));
        }
    }
}
