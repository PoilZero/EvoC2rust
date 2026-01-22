pub fn cb_bitset_to_string(mut bs: Ptr<bitset_t>) -> Ptr<u8> {
    let mut res: Ptr<u8> = c_malloc!(c_sizeof!(u8) * (bs.size + 1));
    c_for!(let mut i: usize = 0; i < bs.size; i.suffix_plus_plus(); {
        res[i] = if cb_bitset_test() { b'1' } else { b'0' };
    });
    let tmp0 = bs.size;
    res[tmp0] = 0u8;
    return res;
}