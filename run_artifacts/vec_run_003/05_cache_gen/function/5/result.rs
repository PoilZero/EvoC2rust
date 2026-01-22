pub fn vec_splice_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut start: i32, mut count: i32) {
    let _ = capacity;
    c_memmove!(
        (c_ref!(data).cast::<Ptr<u8>>() + start * memsz).cast(),
        (c_ref!(data).cast::<Ptr<u8>>() + (start + count) * memsz).cast(),
        ((length.cast() - start - count) * memsz).cast::<usize>()
    );
}