pub fn cb_bitset_any(mut bs: Ptr<bitset_t>) -> bool {
    if (bs.flip).as_bool() {
        bs.count != bs.size
    } else {
        bs.count != 0
    }
}