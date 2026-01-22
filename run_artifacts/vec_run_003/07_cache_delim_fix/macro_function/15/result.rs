macro_rules! vec_reserve {
    ($v:expr, $n:expr) => {
        vec_reserve_(vec_unpack_($v.cast()), $n.cast())
    }
}
pub(crate) use vec_reserve;