macro_rules! vec_swap {
    ($v:expr, $idx1:expr, $idx2:expr) => {
        vec_swap_!(vec_unpack_!($v).cast(), $idx1.cast(), $idx2.cast())
    }
}
pub(crate) use vec_swap;