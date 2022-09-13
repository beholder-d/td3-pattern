use std::convert::TryInto;
use std::error::Error;
use std::str::FromStr;
use std::str::Lines;

extern crate scan_fmt;
use scan_fmt::scan_fmt;

use crate::step;

pub struct Pattern {
    triplet: bool,
    active_steps: u8,
    step: [step::Step; 16],
}

impl Default for Pattern {
    fn default() -> Pattern {
        Pattern { triplet: false, active_steps: 1, step: [step::Step { ..Default::default() }; 16] }
    }
}

// takes 4 lower bits of entries in u8 array msg[$start -> $start+4] and arranges it in u16 like s2s3s0s1
macro_rules! four_u8_to_u16 {
    ($msg: expr, $start: expr) => {
        (($msg[$start] as u16) << 4)
            + $msg[$start + 1] as u16
            + (($msg[$start + 2] as u16) << 12)
            + (($msg[$start + 3] as u16) << 8)
    };
}

pub fn sysex_to_pattern(msg: &[u8]) -> Pattern {
    let tienum = four_u8_to_u16!(msg, 0x6B);
    let restnum = four_u8_to_u16!(msg, 0x6F);
    let mut step: [step::Step; 16] = Default::default();
    for n in 0..16 {
        let s = &mut step[n];
        // whether it's upper c (i.e. last bit is 1)
        let dn = n * 2;
        let note = (msg[0x06 + dn] + (msg[0x05 + dn] << 4)) & 0x7f;
        let mut upperc = (msg[0x05 + dn] & 0x8) >> 3;
        // also there is case, when it's done via midi number for upper C
        if note == 0x30 {
            upperc = 01;
        }
        // we're actually using 13 notes like it's on td-3
        s.note = note % 12 + upperc * 12;
        s.transpose = (note / 12 - 1 - upperc).try_into().unwrap();
        s.accent = msg[0x26 + dn].try_into().unwrap();
        s.slide = msg[0x46 + dn].try_into().unwrap();
        // rest is more important than tie in sequencor
        s.time = (((&tienum & (1 as u16) << n) >> n) + (((&restnum & (1 as u16) << n) >> n) << 1)).try_into().unwrap();
        if cfg!(debug_assertions) {
            println!(
                "{:02}: raw nt {:03} - (c^={:1}) mid {:02} - nt {:02} tr {:?}, ac {}, sl {}, raw t {} r {} - t/r {:?}",
                n,
                (msg[0x06 + dn] + (msg[0x05 + dn] << 4)),
                upperc,
                note,
                s.note,
                s.transpose,
                s.slide as u16,
                s.accent as u16,
                (&tienum & (1 as u16) << n) >> n,
                (&restnum & (1 as u16) << n) >> n,
                s.time
            );
        }
    }
    Pattern { triplet: msg[0x66] == 1, active_steps: (msg[0x67] << 4) + msg[0x68], step }
}

const TD3_PATTERN: &'static str = "TD-3 Pattern";
const ACTIVE_STEPS: &'static str = "Active Steps";
const TRIPLET: &'static str = "Triplet Time";
const NOTE_S: &'static str = "Note:      ";
const TRANSPOSE_S: &'static str = "Transpose: ";
const ACCENT_S: &'static str = "Accent:    ";
const SLIDE_S: &'static str = "Slide:     ";
const TIME: &'static str = "Tie/Rest:  ";

const NOTE: &'static [&str] = &["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B", "C^"];

pub fn pattern_to_string(pattern: &Pattern) -> String {
    let mut sep = String::from("");
    let mut num = String::from("// Step:   ");
    let mut note = String::from(NOTE_S);
    let mut transpose = String::from(TRANSPOSE_S);
    let mut accent = String::from(ACCENT_S);
    let mut slide = String::from(SLIDE_S);
    let mut time = String::from(TIME);
    for i in 0..=15 {
        if i == 1 {
            sep.push(',');
        }
        let s = &pattern.step[i];
        num.push_str(&format!("{} {:02?}", sep, i + 1));
        note.push_str(&format!("{} {:2}", sep, NOTE[s.note as usize]));
        transpose.push_str(&format!("{} {:2?}", sep, s.transpose));
        accent.push_str(&format!("{} {:2?}", sep, s.accent));
        slide.push_str(&format!("{} {:2?}", sep, s.slide));
        time.push_str(&format!("{} {:2?}", sep, s.time));
    }
    num.push('\n');
    note.push_str("  // C -C# .. B -C^\n");
    transpose.push_str("  // DN-  -UP\n");
    accent.push_str("  //   -AC\n");
    slide.push_str("  //   -SL\n");
    time.push_str("  //   -TI-RE\n");

    let mut pattern_str = String::from(TD3_PATTERN);
    pattern_str.push('\n');
    pattern_str.push_str(&format!(
        "{}: {}, {}: {}\n",
        ACTIVE_STEPS,
        pattern.active_steps,
        TRIPLET,
        if pattern.triplet { "On" } else { "Off" }
    ));
    pattern_str.push('\n');
    pattern_str.push_str(&num);
    pattern_str.push_str(&note);
    pattern_str.push_str(&transpose);
    pattern_str.push_str(&accent);
    pattern_str.push_str(&slide);
    pattern_str.push_str(&time);
    pattern_str.push_str("// Read 'Sequencer Quirks' in README.md about Tie/Rest and pattern execution\n");

    pattern_str
}

pub fn next_nonempty_line(lines: &mut Lines) -> String {
    let line = "";
    while let Some(line) = lines.next() {
        let line = match line.find("//") {
            Some(x) => line[0..x].trim(),
            None => line.trim(),
        };
        if line.len() > 0 {
            return line.to_string();
        }
    }
    line.to_owned()
}

// Originally this funciton was intended to return Vec<&str> and without to_owned but it was causing
// cannot return value referencing local variable rust? is there a way how to keep line borrowed
fn split_entries<'a>(lines: &mut Lines, start: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let line = next_nonempty_line(lines);
    if !line.starts_with(start) {
        return Err(format!("Expecting {}: ..., read: {}", start, line).into());
    }
    let vals: Vec<String> =
        (&line[start.len()..]).split(",").collect::<Vec<&str>>().iter().map(|x| x.trim().to_owned()).collect();

    if vals.len() != 16 {
        return Err(format!("Line with '{}' should have 16 values, {} found instead", start.trim(), vals.len()).into());
    }
    Ok(vals)
}

pub fn string_to_pattern(string_pattern: String) -> Result<Pattern, Box<dyn Error>> {
    let mut pattern: Pattern = Default::default();
    let mut lines = string_pattern.lines();
    // TD-3 Pattern
    let line = next_nonempty_line(&mut lines);
    if line != TD3_PATTERN {
        return Err(format!("Expecting {}, read: {}", TD3_PATTERN, line).into());
    }
    // Active Steps
    let line = next_nonempty_line(&mut lines);
    match scan_fmt!(&line, "{[^:]}: {d}, {[^:]}: {}", String, u8, String, String) {
        Err(_) => return Err(format!("Expecting {}: # and {}: #, read: {}", ACTIVE_STEPS, TRIPLET, line).into()),
        Ok((a_s, a_s_value, trip, trip_value)) => {
            if a_s != ACTIVE_STEPS || trip != TRIPLET {
                return Err(format!("Expecting {}: # and {}: #, read: {}", ACTIVE_STEPS, TRIPLET, line).into());
            }
            pattern.active_steps = a_s_value;
            pattern.triplet = trip_value == "On";
        }
    }
    let note = split_entries(&mut lines, NOTE_S)?;
    let transpose = split_entries(&mut lines, TRANSPOSE_S)?;
    let accent = split_entries(&mut lines, ACCENT_S)?;
    let slide = split_entries(&mut lines, SLIDE_S)?;
    let time = split_entries(&mut lines, TIME)?;
    for i in 0..=15 {
        let s = &mut pattern.step[i];
        // todo: turn to macro or fn and callbacks
        match NOTE.iter().position(|&n| note[i] == n) {
            Some(x) => s.note = x as u8,
            None => return Err(format!("Wrong '{}' on postion {}: {}", NOTE_S.trim(), i, note[i]).into()),
        };
        match step::Transpose::from_str(&transpose[i]) {
            Ok(x) => s.transpose = x,
            _ => return Err(format!("Wrong '{}' on postion {}: {}", TRANSPOSE_S.trim(), i, transpose[i]).into()),
        };
        match step::Accent::from_str(&accent[i]) {
            Ok(x) => s.accent = x,
            _ => return Err(format!("Wrong '{}' on postion {}: {}", ACCENT_S.trim(), i, accent[i]).into()),
        };
        match step::Slide::from_str(&slide[i]) {
            Ok(x) => s.slide = x,
            _ => return Err(format!("Wrong '{}' on postion {}: {}", SLIDE_S.trim(), i, slide[i]).into()),
        };
        match step::Time::from_str(&time[i]) {
            Ok(x) => s.time = x,
            _ => return Err(format!("Wrong '{}' on postion {}: {}", TIME.trim(), i, time[i]).into()),
        };
    }
    Ok(pattern)
}

macro_rules! u16_to_four_u8 {
    ($var: expr) => {
        [
            (($var & 0x00F0) >> 4) as u8,
            ($var & 0x000F) as u8,
            (($var & 0xF000) >> 12) as u8,
            (($var & 0x0F00) >> 8) as u8,
        ]
    };
}

// output size should be 115 bytes
pub fn pattern_to_sysex(pattern: &Pattern, group: u8, pnum: u8, ab: u8) -> Vec<u8> {
    let mut note: [u8; 32] = [0; 32];
    let mut accent: [u8; 32] = [0; 32];
    let mut slide: [u8; 32] = [0; 32];
    let mut tie = 0u16;
    let mut rest = 0u16;
    for i in 0..=15 {
        let d = i << 1;
        let s = &pattern.step[i];
        let hbit = if s.note >= 12 { 0x80 } else { 0 };
        let composed_note: u8 = 12 + s.note + ((s.transpose as u8) * 12) + hbit;
        note[d] = (composed_note & 0b11110000) >> 4;
        note[d + 1] = composed_note & 0b00001111;
        accent[d + 1] = s.accent as u8;
        slide[d + 1] = s.slide as u8;
        tie = tie + (((s.time as u16) & 0b01) << i);
        rest = rest + ((((s.time as u16) & 0b10) >> 1) << i);
        if cfg!(debug_assertions) {
            println!("{:02}: t {} r {}", i, ((s.time as u16) & 0b01), (((s.time as u16) & 0b10) >> 1));
        }
    }
    // create sysex
    let mut sysex: Vec<u8> = Vec::new();
    sysex.push(0x78);
    sysex.extend_from_slice(&[group, pnum + (ab << 3)]);
    sysex.extend_from_slice(&[00, 01]);
    sysex.extend_from_slice(&note);
    sysex.extend_from_slice(&accent);
    sysex.extend_from_slice(&slide);
    sysex.extend_from_slice(&[00, pattern.triplet as u8]);
    sysex.extend_from_slice(&[(pattern.active_steps & 0xF0) >> 4, pattern.active_steps & 0x0F]);
    sysex.extend_from_slice(&[00, 00]);
    sysex.extend_from_slice(&(u16_to_four_u8!(tie)));
    sysex.extend_from_slice(&(u16_to_four_u8!(rest)));
    return sysex;
}
