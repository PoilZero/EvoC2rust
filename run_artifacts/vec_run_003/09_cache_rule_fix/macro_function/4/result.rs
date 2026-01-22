macro_rules! vec_push {
    ($v:expr, $val:expr) => {
        if vec_expand_(vec_unpack_($v.cast())) {
            -1
        } else {
            $v.data[$v.length.suffix_plus_plus()] = $val as u8;
            0
        }
    }
}
pub(crate) use vec_push;