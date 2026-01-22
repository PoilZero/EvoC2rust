macro_rules! vec_last { ($v:expr) => {
    $v.cast().data[$v.cast().length - 1]
} }
pub(crate) use vec_last;