pub fn vec_compact_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, memsz: i32) -> i32 {
    if ((*length).cast() == 0).as_bool() {
        c_free!((*data).cast::<Ptr<Void>>());
        *data = NULL!();
        *capacity = 0;
        return 0;
    }

    let n = (*length).cast();
    let new_size = n * memsz;

    let ptr = c_realloc!((*data).cast::<Ptr<Void>>(), new_size as usize);
    if ptr == NULL!() {
        return -1;
    }

    *data = ptr.cast::<Ptr<u8>>();
    *capacity = n;
    return 0;
}