struct Vec<T> {
    data: *mut T,
    length: i32,
    capacity: i32,
}