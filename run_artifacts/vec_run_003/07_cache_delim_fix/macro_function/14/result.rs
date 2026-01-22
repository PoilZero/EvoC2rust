macro_rules! vec_last { ($v:expr) => {
    let idx = $v.length - 1;
    $v.data[idx]
} }
pub(crate) use vec_last;