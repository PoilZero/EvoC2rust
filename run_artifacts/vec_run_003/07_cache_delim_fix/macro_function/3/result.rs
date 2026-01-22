pub(crate) macro_rules! vec_deinit {
    ($v:expr) => {
        unsafe {
            free($v.data.cast::<VoidPtr>());
        }
        vec_init($v);
    }
}