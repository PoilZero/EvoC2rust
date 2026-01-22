use crate::translation_utils::*;
pub use crate::src::bitset_c::cb_bitset_test;
pub use crate::src::bitset_c::cb_bitset_new;
pub use crate::src::bitset_c::cb_bitset_flip;
pub use crate::src::bitset_c::cb_bitset_delete;
pub use crate::src::bitset_c::cb_bitset_count;
pub use crate::src::bitset_c::cb_bitset_none;
pub use crate::src::bitset_c::cb_bitset_reset;
pub use crate::src::bitset_c::cb_bitset_to_string;
pub use crate::src::bitset_c::cb_bitset_all;
pub use crate::src::bitset_c::cb_bitset_read;
pub use crate::src::bitset_c::cb_bitset_any;
pub use crate::src::bitset_c::cb_bitset_size;
pub use crate::src::bitset_c::cb_bitset_write;
pub use crate::src::bitset_c::cb_bitset_set;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct bitset_t {
    pub flip: bool,
    pub size: usize,
    pub count: usize,
    pub array: Ptr<u8>,
}

pub(crate) macro_rules! _CBORG_BITSET_H { () => { cstr!("_CBORG_BITSET_H") } }
pub(crate) use _CBORG_BITSET_H;

macro_rules! __bitset_size { ($nbits:expr) => { ($nbits + 7) >> 3 } }
pub(crate) use __bitset_size;

pub(crate) macro_rules! __bitset_byte {
    ($idx:expr) => {
        $idx >> 3
    }
}
pub(crate) use __bitset_byte;

pub(crate) macro_rules! __bitset_bit_mask { ($idx:expr) => { 1u32 << (($idx & 0x7) as u32) } }
pub(crate) use __bitset_bit_mask;

