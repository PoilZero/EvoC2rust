pub(crate) macro_rules! vec_truncate {
    ($v:expr, $len:expr) => {
        $v.cast().length = if $len < $v.cast().length { $len } else { $v.cast().length };
    };
}
pub(crate) use vec_truncate;