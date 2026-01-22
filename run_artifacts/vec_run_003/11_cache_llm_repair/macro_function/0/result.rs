macro_rules! vec_unpack_ {
    ($v:expr) => {
        (
            c_ref!($v.data).cast::<Ptr<Ptr<u8>>>(),
            c_ref!($v.length),
            c_ref!($v.capacity),
            c_sizeof!(u8)
        )
    }
}
pub(crate) use vec_unpack_;