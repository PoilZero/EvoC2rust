pub fn vec_reserve_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut n: i32) -> i32 {
    let _ = length;
    if (n > capacity.cast()) {
        let mut ptr: Ptr<Void> = c_realloc!(data, n * memsz);
        if (ptr == NULL!()) {
            return -1;
        }
        data = ptr.cast();
        capacity = n;
    }
    return 0;
}