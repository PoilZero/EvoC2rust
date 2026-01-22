pub(crate) macro_rules! vec_deinit {
    ($v:expr) => {
        free($v.data.cast::<VoidPtr>());
        vec_init($v.cast());
    }
}
pub(crate) use vec_deinit;