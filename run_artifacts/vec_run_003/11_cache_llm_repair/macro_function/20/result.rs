pub(crate) macro_rules! vec_remove {
    ($v:expr, $val:expr) => {
        let mut idx__: i32;
        vec_find($v.cast(), $val.cast(), idx__.cast());
        if idx__ != -1 {
            vec_splice($v.cast(), idx__.cast(), 1.cast());
        }
    };
}
pub(crate) use vec_remove;