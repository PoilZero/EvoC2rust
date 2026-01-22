pub fn vec_insert_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut idx: i32) -> i32 {
    let mut err: i32 = vec_expand_(data.cast(), length.cast(), capacity.cast(), memsz);
    if err != 0 {
        return err;
    }
    c_memmove!((data.cast() + (idx + 1) * memsz).cast(), (data.cast() + idx * memsz).cast(), ((length.cast() - idx) * memsz).cast::<u32>());
    return 0;
}