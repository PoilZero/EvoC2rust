pub fn vec_expand_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32) -> i32 {
    if (c_ref!(length).cast::<i32>() + 1 > c_ref!(capacity).cast::<i32>()) {
        let mut ptr: Ptr<Void> = Default::default();
        let mut n: i32 = if c_ref!(capacity).cast::<i32>() == 0 {
            1
        } else {
            c_ref!(capacity).cast::<i32>() << 1
        };
        ptr = c_realloc!(c_ref!(data).cast::<Ptr<Void>>(), (n * memsz));
        if (ptr == NULL!()) {
            return -1;
        }
        let mut data_ptr = c_ref!(data).cast::<Ptr<u8>>();
        data_ptr = ptr.cast::<Ptr<u8>>();
        let mut capacity_ptr = c_ref!(capacity).cast::<i32>();
        capacity_ptr = n;
    }
    return 0;
}