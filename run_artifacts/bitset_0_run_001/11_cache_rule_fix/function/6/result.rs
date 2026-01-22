pub fn cb_bitset_none(mut bs: Ptr<bitset_t>) -> bool {
    !cb_bitset_any(bs).cast::<bool>()
}