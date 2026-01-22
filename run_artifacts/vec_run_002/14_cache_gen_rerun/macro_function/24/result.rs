pub(crate) macro_rules! vec_foreach_ptr {
    ($v:expr, $var:expr, $iter:expr) => {
        if $v.length > 0 {
            for iter in 0..$v.length {
                $var = c_ref!($v.data[iter]).cast::<Ptr<u8>>();
            }
        }
    };
}
pub(crate) use vec_foreach_ptr;