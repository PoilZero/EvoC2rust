macro_rules! vec_sort {
    ($v:expr, $fn:expr) => {
        qsort(
            c_ref!($v.data),
            $v.length.cast(),
            c_sizeof!(*c_ref!($v.data)),
            $fn.cast()
        )
    }
}
pub(crate) use vec_sort;