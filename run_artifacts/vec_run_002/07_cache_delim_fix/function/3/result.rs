pub fn vec_compact_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32) -> i32 {
    if (length.cast() == 0).as_bool() {
        let inner_ptr = data.cast();
        c_free!(inner_ptr.cast::<Ptr<Void>>());
        *data = NULL!().cast::<Ptr<u8>>();
        *capacity = 0;
        return 0;
    } else {
        let n: i32 = length.cast();
        let inner_ptr = data.cast();
        let ptr = c_realloc!(inner_ptr.cast::<Ptr<Void>>(), n * memsz as usize);
        if (ptr == NULL!()).as_bool() {
            return -1;
        }
        *data = ptr.cast::<Ptr<u8>>();
        *capacity = n;
        return 0;
    }
}