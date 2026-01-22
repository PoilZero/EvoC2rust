macro_rules! vec_init {
    ($v:expr) => {
        memset($v.cast(), 0, c_sizeofval!(*$v))
    }
}
pub(crate) use vec_init;