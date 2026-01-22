#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct VecInt {
    pub data: Ptr<i32>,
    pub length: u32,
}
pub type VecIntT = VecInt;