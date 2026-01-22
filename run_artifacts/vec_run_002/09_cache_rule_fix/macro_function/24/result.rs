pub(crate) macro_rules! vec_foreach_ptr {
    ($v:expr, $var:expr, $iter:expr) => {
        if $v.length > 0 {
            let mut iter = 0;
            while iter < $v.length {
                $var = c_ref!($v.data[iter]);
                iter = iter.plus_plus();
            }
        }
    }
}
pub(crate) use vec_foreach_ptr;