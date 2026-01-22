struct bitset_t {
    array: Array<u8, 1024>,
    flip: bool,
    count: usize,
}

pub fn cb_bitset_set(mut bs: Ptr<bitset_t>, mut idx: usize, mut value: bool) {
    let flip_value = if c_ref!(bs).cast::<Ptr<bitset_t>>().flip { !value } else { value };
    let previous_count = c_popcount!(c_ref!(bs).cast::<Ptr<bitset_t>>().array[__bitset_byte(idx)]);
    if flip_value {
        c_ref!(bs).cast::<Ptr<bitset_t>>().array[__bitset_byte(idx)] |= __bitset_bit_mask(idx);
    } else {
        c_ref!(bs).cast::<Ptr<bitset_t>>().array[__bitset_byte(idx)] &= !__bitset_bit_mask(idx);
    }
    c_ref!(bs).cast::<Ptr<bitset_t>>().count += (c_popcount!(c_ref!(bs).cast::<Ptr<bitset_t>>().array[__bitset_byte(idx)]) - previous_count);
}