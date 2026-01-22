use crate::translation_utils::*;
pub use crate::src::vec_h::*;

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
        c_ref!(data).cast() = ptr.cast::<Ptr<u8>>();
        c_ref!(capacity).cast() = n;
    }
    return 0;
}

pub fn vec_reserve_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut n: i32) -> i32 {
    let _ = length;
    if (n > capacity.cast()) {
        let mut ptr: Ptr<Void> = c_realloc!(data, n * memsz);
        if (ptr == NULL!()) {
            return -1;
        }
        data = ptr.cast();
        *capacity = n;
    }
    return 0;
}

pub fn vec_reserve_po2_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut n: i32) -> i32 {
    let mut n2: i32 = 1;
    if (n == 0).as_bool() {
        return 0;
    }
    while (n2 < n).as_bool() {
        n2 = n2 << 1;
    }
    vec_reserve_(data, length, capacity, memsz, n2)
}

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

pub fn vec_insert_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut idx: i32) -> i32 {
    let mut err: i32 = vec_expand_(data, length, capacity, memsz);
    if err != 0 {
        return err;
    }
    c_memmove!((data + (idx + 1) * memsz), (data + idx * memsz), ((length.cast() - idx) as u32 * memsz as u32));
    return 0;
}

pub fn vec_splice_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut start: i32, mut count: i32) {
    let _ = capacity;
    c_memmove!(
        (c_ref!(data).cast::<Ptr<u8>>() + start * memsz),
        (c_ref!(data).cast::<Ptr<u8>>() + (start + count) * memsz),
        ((length.cast::<i32>() - start - count) * memsz) as usize
    );
}

pub fn vec_swapsplice_(
    mut data: Ptr<Ptr<u8>>,
    mut length: Ptr<i32>,
    mut capacity: Ptr<i32>,
    mut memsz: i32,
    mut start: i32,
    mut count: i32
) {
    let _ = capacity;
    c_memmove!(
        (data.cast::<Ptr<u8>>() + start * memsz),
        (data.cast::<Ptr<u8>>() + (length.cast() - count) * memsz).cast(),
        count * memsz
    );
}

pub fn vec_swap_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut idx1: i32, mut idx2: i32) {
    let _ = length;
    let _ = capacity;
    if idx1 == idx2 {
        return;
    }
    let a: Ptr<u8> = data.cast().add(idx1 * memsz);
    let b: Ptr<u8> = data.cast().add(idx2 * memsz);
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

