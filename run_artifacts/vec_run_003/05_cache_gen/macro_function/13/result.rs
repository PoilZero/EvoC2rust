macro_rules! vec_first { ($v:expr) => { c_ref!($v).data[0] as u8 } }
pub(crate) use vec_first;