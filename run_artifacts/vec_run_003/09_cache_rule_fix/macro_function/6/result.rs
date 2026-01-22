macro_rules! vec_splice { ($v:expr, $start:expr, $count:expr) => {
    vec_splice_(vec_unpack_($v.cast()), $start.cast(), $count.cast());
    $v.length -= $count;
} }
pub(crate) use vec_splice;