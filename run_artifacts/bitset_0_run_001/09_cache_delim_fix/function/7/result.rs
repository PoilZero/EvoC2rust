pub fn cb_bitset_all(mut bs: Ptr<bitset_t>) -> bool {
    return if (bs.flip).as_bool() { bs.count == 0 } else { bs.count == bs.size };
}