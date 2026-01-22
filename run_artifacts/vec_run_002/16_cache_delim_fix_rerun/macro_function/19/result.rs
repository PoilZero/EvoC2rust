macro_rules! vec_find { ($v:expr, $val:expr, $idx:expr) => {
    let mut i: i32 = 0;
    while i < $v.length {
        if $v.data[i] == $val {
            break;
        }
        i.suffix_plus_plus();
    }
    if i == $v.length {
        $idx = -1;
    }
} }
pub(crate) use vec_find;