macro_rules! vec_swapsplice { ($v:expr, $start:expr, $count:expr) => {
    vec_swapsplice_($v.cast(), $start, $count);
    $v.length -= $count;
} }
pub(crate) use vec_swapsplice;