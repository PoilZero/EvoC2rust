macro_rules! vec_insert {
    ($v:expr, $idx:expr, $val:expr) => {
        let unpacked_v = vec_unpack_!($v);
        let result = vec_insert_(unpacked_v, $idx);
        if result != 0 {
            -1
        } else {
            let ptr = unpacked_v + $idx;
            c_ref!(ptr) = $val;
            $v.length.suffix_plus_plus();
            0
        }
        $v.length.suffix_plus_plus();
        0
    }
}
pub(crate) use vec_insert;