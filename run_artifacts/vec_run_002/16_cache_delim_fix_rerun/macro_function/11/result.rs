pub(crate) macro_rules! vec_truncate {
    ($v:expr, $len:expr) => {
        let v = $v.cast();
        let v_deref = v.deref();
        v_deref.length = if $len < v_deref.length { $len } else { v_deref.length };
    }
}
pub(crate) use vec_truncate;