use crate::translation_utils::*;
pub use crate::src::vec_h::*;

pub fn vec_expand_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32) -> i32 {
    if (*length + 1 > *capacity) {
        let mut ptr: Ptr<Void> = c_realloc!(*data, (*capacity == 0).then(|| 1).unwrap_or(*capacity << 1) * memsz);
        if (ptr == NULL!()) {
            return -1;
        }
        *data = ptr.cast::<Ptr<u8>>();
        *capacity = (*capacity == 0).then(|| 1).unwrap_or(*capacity << 1);
    }
    return 0;
}

pub fn vec_reserve_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut n: i32) -> i32 {
    let _ = length;
    if n > *capacity {
        let mut ptr: Ptr<Void> = c_realloc!(*data, n * memsz);
        if (ptr == NULL!()) {
            return -1;
        }
        *data = ptr.cast::<Ptr<u8>>();
        *capacity = n;
    }
    return 0;
}

pub fn vec_reserve_po2_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut n: i32) -> i32 {
    let mut n2: i32 = 1;
    if n == 0 {
        return 0;
    }
    while n2 < n {
        n2 <<= 1;
    }
    return vec_reserve_(data.cast(), length.cast(), capacity.cast(), memsz.cast(), n2.cast()).cast();
}

pub fn vec_compact_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32) -> i32 {
    if (*length == 0) {
        c_free!(*data);
        *data = NULL!();
        *capacity = 0;
        return 0;
    } else {
        let mut ptr: Ptr<Void> = Default::default();
        let mut n: i32 = *length;
        ptr = c_realloc!(*data, n * memsz);
        if (ptr == NULL!()) {
            return -1;
        }
        *capacity = n;
        *data = ptr.cast::<Ptr<u8>>();
    }
    return 0;
}

pub fn vec_insert_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut idx: i32) -> i32 {
    let mut err: i32 = vec_expand_(data, length, capacity, memsz);
    if (err != 0) {
        return err;
    }

    let mut src: Ptr<u8> = *data + idx * memsz;
    let mut dst: Ptr<u8> = *data + (idx + 1) * memsz;
    let count: i32 = *length - idx;

    c_memmove!(dst, src, count * memsz);

    return 0;
}


pub fn vec_splice_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut start: i32, mut count: i32) {
    c_memmove!((*data + start * memsz), (*data + (start + count) * memsz), ((*length - start - count) * memsz));
}

pub fn vec_swapsplice_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut start: i32, mut count: i32) {
    let _ = capacity;
    c_memmove!(data.cast::<Ptr<u8>>() + start * memsz, data.cast::<Ptr<u8>>() + (*length - count) * memsz, count * memsz);
}

pub fn vec_swap_(mut data: Ptr<Ptr<u8>>, mut length: Ptr<i32>, mut capacity: Ptr<i32>, mut memsz: i32, mut idx1: i32, mut idx2: i32) {
    let mut a: Ptr<u8> = (data.cast::<Ptr<u8>>() + idx1 * memsz);
    let mut b: Ptr<u8> = (data.cast::<Ptr<u8>>() + idx2 * memsz);
    let mut count: i32 = memsz;
    let _ = length;
    let _ = capacity;
    if idx1 == idx2 {
        return;
    }
    while count > 0 {
        let tmp: u8 = *a;
        *a = *b;
        *b = tmp;
        a += 1;
        b += 1;
        count -= 1;
    }
}

