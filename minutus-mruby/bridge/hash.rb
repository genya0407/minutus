puts <<~CODE
minu_value minu_hash_keys(minu_state * mrb, minu_value hash) {
  return mrb_hash_keys(mrb, hash);
}
minu_value minu_hash_values(minu_state * mrb, minu_value hash) {
  return mrb_hash_values(mrb, hash);
}
minu_int minu_hash_size(minu_state *mrb, minu_value hash) {
  return mrb_hash_size(mrb, hash);
}
minu_value minu_hash_new_capa(minu_state *mrb, minu_int capa) {
  return mrb_hash_new_capa(mrb, capa);
}
void minu_hash_set(minu_state *mrb, minu_value hash, minu_value key, minu_value val) {
  return mrb_hash_set(mrb, hash, key, val);
}
CODE