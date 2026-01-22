macro_rules! vec_compact { ($v:expr) => {
    vec_compact_(vec_unpack_($v.cast()))
} }
pub(crate) use vec_compact;