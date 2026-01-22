macro_rules! vec_swapsplice { ($v:expr, $start:expr, $count:expr) => {
    vec_swapsplice_(vec_unpack_($v), $start, $count);
    $v.length -= $count;
} }
pub(crate) use vec_swapsplice;