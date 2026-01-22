macro_rules! vec_find { ($v:expr, $val:expr, $idx:expr) => {
    let mut idx = $idx;
    c_for!(idx = 0; idx < $v.length; idx.suffix_plus_plus()) {
        if c_ref!($v.data[idx]) as u8 == $val {
            break;
        }
    }
    if idx == $v.length {
        $idx = -1;
    }
} }
pub(crate) use vec_find;