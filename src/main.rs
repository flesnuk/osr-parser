extern crate glob;
extern crate notify;

use notify::{RecommendedWatcher, Watcher, RecursiveMode,DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::process::Command;
use glob::glob;

mod osr;
mod osudb;


const NONE: u32 = 0;
const NO_FAIL: u32 = 1;
const EASY: u32 = 1 << 1;
const NO_VIDEO: u32 = 1 << 2;
const HIDDEN: u32 = 1 << 3;
const HARD_ROCK: u32 = 1 << 4;
const SUDDEN_DEATH: u32 = 1 << 5;
const DOUBLE_TIME: u32 = 1 << 6;
const RELAX: u32 = 1 << 7;
const HALF_TIME: u32 = 1 << 8;
/// Only set along with [`DOUBLE_TIME`]. i.e.: NC only gives 576
///
/// [`DOUBLE_TIME`]: constant.DOUBLE_TIME.html
const NIGHTCORE: u32 = 1 << 9;
const FLASHLIGHT: u32 = 1 << 10;
const AUTOPLAY: u32 = 1 << 11;
const SPUN_OUT: u32 = 1 << 12;
/// Could be "Autopilot".
const RELAX2: u32 = 1 << 13;
const PERFECT: u32 = 1 << 14;



fn watch() -> notify::Result<()> {
    let path = Path::new("D:/osu!/osu!.db");

    let mut file = File::open(&path).expect("couldn't open db file");

    let length = file.metadata().unwrap().len() as usize;
    let mut buf = vec![0; length];
    let mut p = 0;

    file.read_exact(&mut buf).expect("failed buffering");
    drop(file);
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
    let hsh = hashmap.get(replay.beatmap_hash).expect(
        "lol",
    );

    let rut = format!("{}{}*", "D:/osu!/Songs/", hsh.id);
    println!("{}", rut);

    let mut ruta: std::path::PathBuf = std::path::PathBuf::new();
    for entry in glob(&*rut).expect("Failed to read glob pattern") {
        ruta = entry.expect("");
    }


    let path = format!("{}/{}", ruta.display(), hsh.filename);
    println!("{}", path);
    let output = Command::new("./oppai/oppai.exe")
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
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(1)));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch("D:/osu!/Data/r", RecursiveMode::Recursive));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        let ev = rx.recv().expect("");
        match ev {
            DebouncedEvent::Create(path) => {
                let xx = path.to_str().expect("");
                if xx.ends_with("osr") {
                    let mut file = File::open(&xx).expect("couldn't OPEN osr file");

                    let length = file.metadata().unwrap().len() as usize;
                    let mut buff3 = vec![0; length];
                    let mut pp3 = 0;

                    file.read_exact(&mut buff3).expect("failed buffering");

                    let replay2 = osr::read_replay(&mut pp3, &buff3);
                    let hsh2 = hashmap.get(replay2.beatmap_hash).expect(
                        "lol",
                    );
                    let rut2 = format!("{}{}*", "D:/osu!/Songs/", hsh2.id);
                    let mut ruta2: std::path::PathBuf = std::path::PathBuf::new();
                    for entry in glob(&*rut2).expect("Failed to read glob pattern") {
                        ruta2 = entry.expect("");
                    }

                    let mut NOMOD = replay2.mods == 0;
                    let mut HR = String::from("");
                    let mut DT = String::from("");
                    let mut HD = String::from("");

                    if replay2.mods & HARD_ROCK > 0 {
                        HR = String::from("HR");
                    }

                    if replay2.mods & DOUBLE_TIME > 0 {
                        DT = String::from("DT");
                    }

                    if replay2.mods & HIDDEN > 0 {
                        HD = String::from("HD");
                    }

                    let iku = format!("+{}{}{}", HR, HD, DT);

                    println!("{}", iku);
                    ruta2.push(hsh2.filename);

                    let pathx = format!("{}", ruta2.display());
                    println!("{}",pathx);
                    let output2 = {
                        if NOMOD {
                            Command::new("./oppai/oppai.exe")
                                .arg(pathx)
                                .arg(format!("{}{}", replay2.n100, "x100"))
                                .arg(format!("{}{}", replay2.n50, "x50"))
                                .arg(format!("{}{}", replay2.misses, "xm"))
                                .arg(format!("{}{}", replay2.combo, "x"))
                                .output()
                                .expect("failed to execute process")
                        } else {
                            Command::new("./oppai/oppai.exe")
                                .arg(pathx)
                                .arg(iku)
                                .arg(format!("{}{}", replay2.n100, "x100"))
                                .arg(format!("{}{}", replay2.n50, "x50"))
                                .arg(format!("{}{}", replay2.misses, "xm"))
                                .arg(format!("{}{}", replay2.combo, "x"))
                                .output()
                                .expect("failed to execute process")
                        }
                    };

                    let pp2 = String::from(
                        String::from_utf8_lossy(&output2.stdout)
                            .lines()
                            .last()
                            .expect("failed to get last line from oppai"),
                    );

                    println!("{}", pp2);
                }
            },
            _ => {}
        }
        //match rx.recv() {
        //    Ok(event) => println!("{:?}", event),
        //    Err(e) => println!("watch error: {:?}", e),
        //}
    }
}

fn main() {


    if let Err(e) = watch() {
        println!("error: {:?}", e)
    }

}
