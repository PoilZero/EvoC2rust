pub fn cb_bitset_reset(mut bs: Ptr<bitset_t>) {
    c_memset_s!(bs.array, __bitset_size(bs.size), 0, __bitset_size(bs.size)).cast::<Void>();
    bs.flip = false;
    bs.count = 0;
}