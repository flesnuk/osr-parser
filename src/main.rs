use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;

fn leb128(p: &mut usize, buf: &Vec<u8>) -> usize {
    let mut result: usize = 0;
    let mut shift: u8 = 0;
    loop {
        let byte = &buf[*p];
        *p += 1;
        result = result | ((byte & 0x7f) as usize) << shift;
        if byte & 0x80 == 0x00 {
            break;
        }
        shift += 7;
    }
    return result;
}

fn osu_string<'a>(p: &mut usize, buf: &'a Vec<u8>) -> &'a str {
    let mut start = *p;
    let mut end = *p;
    let byte = &buf[*p];
    *p += 1;
    if *byte == 0x0b {
        let length = leb128(p, buf);
        start = *p;
        end = start + length;
        *p = end;
    }

    let ret = match str::from_utf8(&buf[start..end]) {
        Ok(result) => result,
        Err(why) => panic!("Invalid UTF8 {}", why.description()),
    };
    ret
}

fn main() {
    let path = Path::new(
        "/home/dani/code/rust/src/a3da6995a722f9a674a88f8892401e61-131472766186903799.osr",
    );
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let length = file.metadata().unwrap().len() as usize;
    let mut buf = vec![0; length];
    let _ = file.read_exact(&mut buf);

    let mut p: usize = 0;

    let mut buff = vec![0x0b, 4, 240, 159, 146, 150];

    println!("{}", osu_string(&mut p, &mut buff));
}
