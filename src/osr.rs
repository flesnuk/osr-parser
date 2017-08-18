use std::str;
use std::error::Error;

#[repr(i8)]
#[derive(Debug)]
pub enum BeatmapMode {
    Standard = 0,
    Taiko = 1,
    CatchTheBeat = 2,
    Mania = 3,
}

impl BeatmapMode {
    fn from_byte(b: u8) -> BeatmapMode {
        unsafe { ::std::mem::transmute(b) }
    }
}

#[derive(Debug)]
pub struct OsuReplay<'a> {
    ///Game mode of the replay
    game_mode: BeatmapMode,
    ///Version of the game when the replay was created (ex. 20131216)
    game_version: u32,
    ///osu! beatmap MD5 hash
    beatmap_hash: &'a str,
    ///Player name
    player_name: &'a str,
    ///osu! replay MD5 hash (includes certain properties of the replay)
    replay_hash: &'a str,
    ///Number of 300s
    n300: u16,
    ///Number of 100s in standard, 150s in Taiko, 100s in CTB, 200s in mania
    n100: u16,
    ///Number of 50s in standard, small fruit in CTB, 50s in mania
    n50: u16,
    ///Number of Gekis in standard, Max 300s in mania
    geki: u16,
    ///Number of Katus in standard, 100s in mania
    katu: u16,
    ///Number of misses
    misses: u16,
    ///Total score displayed on the score report
    score: u32,
    ///Greatest combo displayed on the score report
    combo: u16,
    ///Perfect/full combo (1 = no misses and no slider breaks and no early finished sliders)
    perfect: u8,
    ///Mods used.
    mods: u32,
}

fn read_byte(p: &mut usize, buf: &Vec<u8>) -> u8 {
    let byte = &buf[*p];
    *p += 1;
    *byte
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
    for i in 1..7 {
        long |= ((read_byte(p, buf)) as u64) << (8 * i);
    }
    long
}

fn read_uleb128(p: &mut usize, buf: &Vec<u8>) -> usize {
    let mut result: usize = 0;
    let mut shift: u8 = 0;
    loop {
        let byte = read_byte(p, buf);
        result = result | ((byte & 0x7f) as usize) << shift;
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
        *p = end;
    }

    let ret = match str::from_utf8(&buf[start..end]) {
        Ok(result) => result,
        Err(why) => panic!("Invalid UTF8 {}", why.description()),
    };
    ret
}

pub fn read_replay<'a>(p: &mut usize, buf: &'a Vec<u8>) -> OsuReplay<'a> {
    OsuReplay {
        game_mode: BeatmapMode::from_byte(read_byte(p, buf)),
        game_version: read_int(p, buf),
        beatmap_hash: read_osu_string(p, buf),
        player_name: read_osu_string(p, buf),
        replay_hash: read_osu_string(p, buf),
        n300: read_short(p, buf),
        n100: read_short(p, buf),
        n50: read_short(p, buf),
        geki: read_short(p, buf),
        katu: read_short(p, buf),
        misses: read_short(p, buf),
        score: read_int(p, buf),
        combo: read_short(p, buf),
        perfect: read_byte(p, buf),
        mods: read_int(p, buf),
    }
}
