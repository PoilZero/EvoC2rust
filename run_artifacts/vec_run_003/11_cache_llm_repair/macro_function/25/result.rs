macro_rules! vec_foreach_ptr_rev { ($v:expr, $var:expr, $iter:expr) => {
    if $v.length > 0 {
        let mut $iter = $v.length - 1;
        while $iter >= 0 {
            let idx = $iter;
            $var = c_ref!($v.data[idx]);
            $iter.minus_minus();
        }
    }
} }
pub(crate) use vec_foreach_ptr_rev;