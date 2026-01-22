pub fn cb_bitset_count(bs: Ptr<bitset_t>) -> usize {
    if bs.flip {
        bs.size - bs.count
    } else {
        bs.count
    }
}