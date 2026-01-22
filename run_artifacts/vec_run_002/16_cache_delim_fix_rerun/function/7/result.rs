pub fn vec_swap_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut idx1: i32, mut idx2: i32) {
    let _ = length;
    let _ = capacity;
    if idx1 == idx2 {
        return;
    }
    let a: Ptr<u8> = data.cast().add(idx1 * memsz).cast();
    let b: Ptr<u8> = data.cast().add(idx2 * memsz).cast();
    let mut count: i32 = memsz;
    while count > 0 {
        let tmp: u8 = *a;
        *a = *b;
        *b = tmp;
        a = a.suffix_plus_plus();
        b = b.suffix_plus_plus();
        count = count.suffix_minus_minus();
    }
}