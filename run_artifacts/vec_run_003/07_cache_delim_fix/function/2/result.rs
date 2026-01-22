pub fn vec_reserve_po2_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut n: i32) -> i32 {
    let mut n2: i32 = 1;
    if (n == 0).as_bool() {
        return 0;
    }
    while (n2 < n).as_bool() {
        n2 = n2 << 1;
    }
    vec_reserve_(data.cast(), length.cast(), capacity.cast(), memsz.cast(), n2.cast())
}