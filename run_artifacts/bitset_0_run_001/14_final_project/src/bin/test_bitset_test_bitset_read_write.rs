use bitset::bitset::Bitset;
use std::fs;
use std::path::Path;
fn main() {}
#[test]
fn test_bitset_read_write() {
    let mut bsw = Bitset::new(32);
    bsw.set(9, true);
    bsw.set(31, true);
    bsw.flip();
    let path = "./bitset.bs";
    bsw.write(path).unwrap();
    let bsr = Bitset::read(path).unwrap();
    assert_eq!(bsr.flip, bsw.flip);
    
    
    
    fs::remove_file(path).unwrap();
}
