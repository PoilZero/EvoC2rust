macro_rules! __bitset_size { ($nbits:expr) => { ($nbits + 7) >> 3 } }
pub(crate) use __bitset_size;