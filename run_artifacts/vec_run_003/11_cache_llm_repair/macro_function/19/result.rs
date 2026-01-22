macro_rules! vec_find { ($v:expr, $val:expr, $idx:expr) => {
    let mut idx = $idx;
    for i in 0..$v.length {
        if c_ref!($v.data[i]) == $val {
            idx = i;
            break;
        }
    }
    if idx == $v.length {
        idx = -1;
    }
} }
pub(crate) use vec_find;