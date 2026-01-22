pub(crate) macro_rules! __bitset_bit_mask { ($idx:expr) => { 1u32 << (($idx & 0x7) as u32) } }
pub(crate) use __bitset_bit_mask;