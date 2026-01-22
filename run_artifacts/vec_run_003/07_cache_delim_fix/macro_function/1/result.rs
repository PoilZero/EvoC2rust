pub(crate) macro_rules! vec_t {
    ($t:ty) => {
        struct Vec {
            data: Ptr<$t>,
            length: i32,
            capacity: i32,
        }
    };
}
pub(crate) use vec_t;