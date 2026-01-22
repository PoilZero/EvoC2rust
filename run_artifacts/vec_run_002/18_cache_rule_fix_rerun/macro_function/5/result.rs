macro_rules! vec_pop {
    ($v:expr) => {
        let idx = minus_minus!($v.length);
        $v.data[idx]
    }
}
pub(crate) use vec_pop;