use std::io::Read;

pub fn get_char() -> u16 {
    std::io::stdin().bytes().next().unwrap().unwrap() as u16
}
