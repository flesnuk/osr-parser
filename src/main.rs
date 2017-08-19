extern crate glob;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::process::Command;
use glob::glob;

mod osr;
mod osudb;

fn main() {
    let path = Path::new("/media/dani/DATA/osu!/osu!.db");

    let mut file = File::open(&path).expect("couldn't open osr file");

    let length = file.metadata().unwrap().len() as usize;
    let mut buf = vec![0; length];
    let mut p = 0;

    file.read_exact(&mut buf).expect("failed buffering");

    let hashmap = osudb::read_osudb(&mut p, &buf);

    let path = Path::new(
        "./test/a3da6995a722f9a674a88f8892401e61-131472766186903799.osr",
    );

    let mut file = File::open(&path).expect("couldn't open osr file");

    let length = file.metadata().unwrap().len() as usize;
    let mut buff = vec![0; length];
    let mut pp = 0;

    file.read_exact(&mut buff).expect("failed buffering");

    let replay = osr::read_replay(&mut pp, &buff);

    println!("{:?}", replay);
    let hsh = hashmap.get("a3da6995a722f9a674a88f8892401e61").expect(
        "lol",
    );

    let rut = format!("{}{}*", "/media/dani/DATA/osu!/Songs/", hsh.id);
    println!("{}", rut);

    let mut ruta: std::path::PathBuf = std::path::PathBuf::new();
    for entry in glob(&*rut).expect("Failed to read glob pattern") {
        ruta = entry.expect("");
    }

    let path = format!("{}/{}", ruta.display(), hsh.filename);
    println!("{}", path);
    let output = Command::new("./oppai/oppai")
        .arg(path)
        .arg("+HR")
        .arg(format!("{}{}", replay.n100, "x100"))
        .arg(format!("{}{}", replay.n50, "x50"))
        .arg(format!("{}{}", replay.misses, "xm"))
        .arg(format!("{}{}", replay.combo, "x"))
        .output()
        .expect("failed to execute process");

    let pp = String::from(
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .last()
            .expect("failed to get last line from oppai"),
    );

    println!("{}", pp);

}
