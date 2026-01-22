macro_rules! vec_insert {
    ($v:expr, $idx:expr, $val:expr) => {
        let unpacked = vec_unpack_($v.cast());
        let result = vec_insert_(unpacked, $idx);
        if result != 0 {
            -1
        } else {
            let idx = $idx;
            let data = c_ref!($v.data);
            data[idx] = $val;
            $v.length.suffix_plus_plus();
            $v.length.suffix_plus_plus();
            0
        }
    };
}
pub(crate) use vec_insert;