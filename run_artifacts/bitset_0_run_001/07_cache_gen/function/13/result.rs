pub fn cb_bitset_read(mut path: Ptr<u8>) -> Ptr<bitset_t> {
    let mut file: FilePtr = c_fopen!(path, cstr!("rb"));
    let mut size: usize = 0;
    
    if (file == NULL!()).as_bool() {
        c_fprintf!(stdout, cstr!("%s\n"), c_strerror!(errno));
        c_exit!(EXIT_FAILURE);
    }
    
    c_fread!(size.cast(), c_sizeof!(usize), 1, file);
    let mut bs: Ptr<bitset_t> = cb_bitset_new!(size);
    c_fread!(bs.count.cast(), c_sizeof!(usize), 1, file);
    c_fread!(bs.flip.cast(), c_sizeof!(bool), 1, file);
    
    while c_fread!(bs.array.cast(), c_sizeof!(u8), bs.size, file) > 0 {
        // Do nothing
    }
    
    c_fclose!(file);
    return bs;
}