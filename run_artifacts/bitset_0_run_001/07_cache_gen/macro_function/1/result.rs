pub(crate) macro_rules! __bitset_byte {
    ($idx:expr) => {
        $idx >> 3
    }
}
pub(crate) use __bitset_byte;