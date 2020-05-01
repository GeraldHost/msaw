mod binary;
mod decoder;
mod types;

use binary::module::Module;
use binary::*;
use decoder::Decoder;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let wasm = File::open("./add.wasm").unwrap();
    let reader = BufReader::new(wasm);

    let mut decoder = Decoder::new(reader);
    match Module::decode(&mut decoder) {
        Ok(_) => println!("ok"),
        Err(error) => panic!("oh dear"),
    };
}
