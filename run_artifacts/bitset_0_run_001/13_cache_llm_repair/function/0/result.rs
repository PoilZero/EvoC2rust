use crate::src::include::cborg::bitset_h::bitset_t;

pub fn cb_bitset_new(mut nb_bits: usize) -> Ptr<bitset_t> {
    let mut bs: Ptr<bitset_t> = c_calloc!(1, c_sizeof!(bitset_t));
    if (bs == NULL!()) {
        return NULL!();
    }
    let array_size = __BITSET_SIZE!(nb_bits);
    let mut array: Ptr<u8> = c_calloc!(array_size, c_sizeof!(u8));
    if (array == NULL!()) {
        return NULL!();
    }
    bs.array = array;
    bs.size = nb_bits;
    return bs;
}