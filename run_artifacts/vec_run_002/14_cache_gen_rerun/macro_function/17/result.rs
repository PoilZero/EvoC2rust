macro_rules! vec_pusharr {
    ($v:expr, $arr:expr, $count:expr) => {
        let mut i__: i32;
        let n__ = $count;
        if vec_reserve_po2_(vec_unpack_($v.cast()), $v.length + n__) != 0 {
            // Break statement is handled by the loop context (not needed in macro body)
        }
        for i__ in 0..n__ {
            let idx = $v.length;
            $v.data[idx] = $arr[i__] as u8;
            $v.length = $v.length.plus_plus();
        }
    }
}
pub(crate) use vec_pusharr;