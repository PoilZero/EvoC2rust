pub fn cb_bitset_write(mut bs: Ptr<bitset_t>, mut path: Ptr<u8>) {
    let mut file: FilePtr = c_fopen!(path, "wb");
    let mut bytes_to_write: usize = c_bitset_size!(bs.size);
    let mut bytes_writen: usize = 0;
    let mut total_writen: usize = 0;

    if (file == NULL!()) {
        let error_msg = c_strerr!(errno);
        println!("{}", error_msg);
        std::process::exit(EXIT_FAILURE);
    }

    c_fwrite!(bs.size, c_sizeof!(size_t), 1, file);
    c_fwrite!(bs.count, c_sizeof!(size_t), 1, file);
    c_fwrite!(bs.flip, c_sizeof!(bool), 1, file);
    while (bytes_writen = c_fwrite!(bs.array + total_writen, c_sizeof!(u8), bytes_to_write - total_writen, file)) > 0 {
        total_writen += bytes_writen;
    }

    c_fclose!(file);
}