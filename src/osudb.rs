use std::str;
use std::error::Error;
use std::collections::HashMap;

fn read_byte(p: &mut usize, buf: &Vec<u8>) -> u8 {
    let byte = &buf[*p];
    *p += 1;
    *byte
}

fn skip_byte(p: &mut usize, buf: &Vec<u8>) {
    *p += 1;
}

fn read_short(p: &mut usize, buf: &Vec<u8>) -> u16 {
    let mut short: u16 = (read_byte(p, buf)) as u16;
    short |= ((read_byte(p, buf)) as u16) << 8;
    short
}

fn read_int(p: &mut usize, buf: &Vec<u8>) -> u32 {
    let mut int: u32 = (read_byte(p, buf)) as u32;
    int |= ((read_byte(p, buf)) as u32) << 8;
    int |= ((read_byte(p, buf)) as u32) << 16;
    int |= ((read_byte(p, buf)) as u32) << 24;
    int
}

fn read_long(p: &mut usize, buf: &Vec<u8>) -> u64 {
    let mut long: u64 = (read_byte(p, buf)) as u64;
    for i in 1..8 {
        long |= ((read_byte(p, buf)) as u64) << (8 * i);
    }
    long
}

fn read_float(p: &mut usize, buf: &Vec<u8>) -> f32 {
    read_int(p, buf) as f32
}

fn read_uleb128(p: &mut usize, buf: &Vec<u8>) -> usize {
    let mut result: usize = 0;
    let mut shift: u8 = 0;
    loop {
        let byte = read_byte(p, buf);
        result |= ((byte & 0x7f) as usize) << shift;
        if byte & 0x80 == 0x00 {
            break;
        }
        shift += 7;
    }
    result
}

fn read_osu_string<'a>(p: &mut usize, buf: &'a Vec<u8>) -> &'a str {
    let mut start = *p;
    let mut end = *p;
    let byte = read_byte(p, buf);
    if byte == 0x0b {
        let length = read_uleb128(p, buf);
        start = *p;
        end = start + length;
        *p += length;
    }

    let ret = match str::from_utf8(&buf[start..end]) {
        Ok(result) => result,
        Err(why) => panic!("Invalid UTF8 {}", why.description()),
    };
    ret
}

fn skip_osu_string<'a>(p: &mut usize, buf: &'a Vec<u8>) {
    let byte = read_byte(p, buf);
    if byte == 0x0b {
        let length = read_uleb128(p, buf);
        *p += length;
    }
}

#[derive(Debug)]
pub struct Beatmap<'a> {
    pub id: u32,
    pub filename: &'a str,
}

pub fn read_osudb<'a>(p: &mut usize, buf: &'a Vec<u8>) -> HashMap<&'a str, Beatmap<'a>> {
    let mut hm = HashMap::new();
    let mut bmsetid: u32;
    let mut bm: Beatmap;
    *p += 17;
    skip_osu_string(p, buf);
    let length = read_int(p, buf);
    for _ in 0..length {
        let size_map = read_int(p, buf) as usize;
        let start = *p;

        for _ in 0..7 {
            skip_osu_string(p, buf);
        }

        let hash = read_osu_string(p, buf);
        let filename = read_osu_string(p, buf);
        *p += 39;
        for _ in 0..4 {
            let longi = read_int(p, buf);
            *p += 14 * longi as usize;
        }

        *p += 12;
        let timing_p = read_int(p, buf);
        for _ in 0..timing_p {
            for _ in 0..17 {
                skip_byte(p, buf);
            }
        }
        read_int(p, buf);
        bmsetid = read_int(p, buf);
        bm = Beatmap {
            id: bmsetid,
            filename: filename,
        };
        hm.insert(hash, bm);
        *p = start + size_map;
    }

    hm
}