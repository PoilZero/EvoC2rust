macro_rules! vec_foreach { ($v:expr, $var:expr, $iter:expr) => {
    if $v.length > 0 {
        c_for!($iter = 0; $iter < $v.length; $iter.plus_plus(); {
            $var = $v.data[$iter];
        })
    }
} }
pub(crate) use vec_foreach;