pub fn cb_bitset_reset(mut bs: Ptr<bitset_t>) {
    c_memset_s!(bs.array, cb_bitset_size(bs), 0, cb_bitset_size(bs)).cast::<Void>();
    bs.flip = false;
    bs.count = 0;
}