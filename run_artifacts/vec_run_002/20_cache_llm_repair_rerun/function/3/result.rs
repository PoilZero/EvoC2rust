pub fn vec_compact_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, memsz: i32) -> i32 {
    if ((*length) == 0) {
        c_free!((*data).cast::<Ptr<Void>>());
        *data = NULL!();
        *capacity = 0;
        return 0;
    }

    let n = (*length);
    let new_size = (n * memsz) as usize; // Fixed: Multiply i32 values first, then cast to usize

    let ptr: Ptr<Void> = c_realloc!((*data), new_size);
    if ptr.is_null() {
        return -1;
    }

    *data = ptr.cast::<Ptr<u8>>();
    *capacity = n;
    return 0;
}