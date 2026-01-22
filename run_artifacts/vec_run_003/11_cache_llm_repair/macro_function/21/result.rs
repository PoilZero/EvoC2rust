macro_rules! vec_reverse { ($v:expr) => {
    let mut i__: i32 = ($v).length / 2;
    while i__ > 0 {
        vec_swap($v.cast(), i__, (($v).length - (i__ + 1)).cast());
        i__.minus_minus();
    }
} }
pub(crate) use vec_reverse;