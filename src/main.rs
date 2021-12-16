extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};
mod hardware;
mod proc;
mod utils;

use std::env::args;
use std::fs;
use std::io::BufReader;

use hardware::memory::Memory;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("enter an argument")
    } else {
        let _mem = read_file_to_mem(&args[1]);
    }
}

fn read_file_to_mem(file: &String) -> Memory {
    let content = fs::File::open(file).expect("file not found");
    let mut content = BufReader::new(content);
    let mut origin = content.read_u16::<BigEndian>().unwrap();
    let mut mem = Memory::new();
    loop {
        match content.read_u16::<BigEndian>() {
            Ok(instr) => {
                mem.write(origin, instr);
                origin += 1
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::UnexpectedEof {
                    return mem;
                }
                panic!("{}", err)
            }
        }
    }
}
