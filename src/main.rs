use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;

mod osr;

fn main() {
    let path = Path::new(
        "./test/a3da6995a722f9a674a88f8892401e61-131472766186903799.osr",
    );

    let mut file = File::open(&path).expect("couldn't open osr file");

    let length = file.metadata().unwrap().len() as usize;
    let mut buf = vec![0; length];
    let mut p = 0;

    file.read_exact(&mut buf).expect("failed buffering");

    let replay = osr::read_replay(&mut p, &buf);

    println!("{:#?}", replay);
}
