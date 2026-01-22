macro_rules! vec_push {
    ($v:expr, $val:expr) => {
        if vec_expand_(vec_unpack_($v.cast())) {
            -1
        } else {
            let idx = $v.length;
            $v.data[idx] = $val as u8;
            $v.length.suffix_plus_plus();
            0
        }
    }
}
pub(crate) use vec_push;