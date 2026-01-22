pub fn cb_bitset_all(mut bs: Ptr<bitset_t>) -> bool {
    return if (bs.flip) { bs.count == 0 } else { bs.count == bs.size };
}