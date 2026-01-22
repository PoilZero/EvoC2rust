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
        (data.cast::<Ptr<u8>>() + start * memsz).cast(),
        (data.cast::<Ptr<u8>>() + (*length - count) * memsz).cast(),
        count * memsz
    );
}