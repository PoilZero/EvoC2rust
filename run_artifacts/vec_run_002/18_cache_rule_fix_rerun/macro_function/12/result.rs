macro_rules! vec_clear {
    ($v:expr) => {
        $v.cast().length = 0;
    };
}
pub(crate) use vec_clear;