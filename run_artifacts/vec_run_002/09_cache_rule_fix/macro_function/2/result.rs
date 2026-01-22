macro_rules! vec_init { ($v:expr) => {
    $v.cast().zero(c_sizeofval!($v))
} }
pub(crate) use vec_init;