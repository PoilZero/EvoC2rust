macro_rules! vec_pusharr {
    ($v:expr, $arr:expr, $count:expr) => {
        let n__ = $count;
        if vec_reserve_po2_(vec_unpack_($v), $v.length + n__) != 0 {
            break;
        }
        for i__ in 0..n__ {
            let idx = $v.length;
            $v.data[idx] = $arr[i__] as u8;
            $v.length.suffix_plus_plus();
        }
    }
}
pub(crate) use vec_pusharr;