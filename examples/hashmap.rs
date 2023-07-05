#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref VEC:Vec<u8> = vec![0x18u8, 0x11u8];
    static ref MAP: HashMap<u32, String> = {
        let mut map = HashMap::new();
        map.insert(18, "hury".to_owned());
        map.insert(19, "tom".to_owned());
        map.insert(20, "perter".to_owned());
        map
    };
    static ref PAGE:u32 = mulit(18);
}

fn mulit(i: u32) -> u32 {
    i * 2
}

fn main() {
    // println!("{:?}", *PAGE);
    // println!("{:?}", *VEC);
    println!("{:?}", *MAP);
}
