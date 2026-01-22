macro_rules! vec_foreach_rev { ($v:expr, $var:expr, $iter:expr) => {
    if $v.length > 0 {
        let mut $iter = $v.length - 1;
        while $iter >= 0 {
            let $var = $v.data[$iter];
            $iter -= 1;
        }
    }
} }
pub(crate) use vec_foreach_rev;