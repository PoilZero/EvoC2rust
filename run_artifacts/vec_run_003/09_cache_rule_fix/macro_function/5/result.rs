macro_rules! vec_pop { ($v:expr) => {
    let idx = $v.length.minus_minus();
    *($v.data + idx) as u8
} }
pub(crate) use vec_pop;