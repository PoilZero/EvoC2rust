#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct bitset_t {
    pub flip: bool,
    pub size: usize,
    pub count: usize,
    pub array: Ptr<u8>,
}