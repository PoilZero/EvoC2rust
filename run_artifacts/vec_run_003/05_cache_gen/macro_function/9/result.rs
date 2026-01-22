macro_rules! vec_sort { ($v:expr, $fn:expr) => {
    qsort(
        $v.data.cast(),
        $v.length.cast(),
        c_sizeof!($v.data),
        $fn
    )
} }
pub(crate) use vec_sort;