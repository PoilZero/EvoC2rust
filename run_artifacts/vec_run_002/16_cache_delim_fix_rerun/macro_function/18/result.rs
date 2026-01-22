macro_rules! vec_extend {
    ($v:expr, $v2:expr) => {
        vec_pusharr(
            $v.cast(),
            c_ref!($v2.data),
            $v2.length.cast()
        );
    }
}
pub(crate) use vec_extend;