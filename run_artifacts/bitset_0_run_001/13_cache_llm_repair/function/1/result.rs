use crate::src::include::cborg::bitset_h::bitset_t;

pub fn cb_bitset_delete(mut bs: Ptr<bitset_t>) {
    c_free!(bs.array);
    c_free!(bs);
}