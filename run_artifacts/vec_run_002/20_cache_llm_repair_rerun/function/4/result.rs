pub fn vec_insert_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut idx: i32) -> i32 {
    let mut err: i32 = vec_expand_(data, length, capacity, memsz);
    if err != 0 {
        return err;
    }
    c_memmove!((data + (idx + 1) * memsz), (data + idx * memsz), ((length.cast() - idx) as u32 * memsz as u32));
    return 0;
}