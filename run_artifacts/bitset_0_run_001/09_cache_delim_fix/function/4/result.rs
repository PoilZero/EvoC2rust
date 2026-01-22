pub fn cb_bitset_test(mut bs: Ptr<bitset_t>, mut idx: usize) -> bool {
    bs.flip ^ ((bs.array[__BITSET_BYTE!(idx)] & __BITSET_BIT_MASK!(idx)) != 0)
}