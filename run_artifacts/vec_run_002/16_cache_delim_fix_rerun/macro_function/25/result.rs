macro_rules! vec_foreach_ptr_rev {
    ($v:expr, $var:expr, $iter:expr) => {
        if $v.length > 0 {
            let mut iter = $v.length - 1;
            while iter >= 0 {
                $var = c_ref!($v.data[iter]);
                iter = iter.minus_minus();
            }
        }
    };
}
pub(crate) use vec_foreach_ptr_rev;