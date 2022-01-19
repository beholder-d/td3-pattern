extern crate argmap;
extern crate midir;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::sync::mpsc;

mod config;
mod step;
use config::{Config, Mode, DEFAULT_PORTNAME};
mod midicomm;
use midicomm::{get_pattern, open_ports, send_sysex};
mod pattern;
use pattern::{pattern_to_string, pattern_to_sysex, string_to_pattern, sysex_to_pattern};

const USAGE: &'static str = "
Usage:
    td3pattern [-in=\"name\"] [-out=\"name\"] <group> <pattern><a|b> [-file=filename]
    td3pattern [-in=\"name\"] [-out=\"name\"] upload <group> <pattern><a|b> -file=filename
Where:
    -in=\"name\" -- name of TD-3's midi in port
    -out=\"name\" -- name of TD-3's midi out port
    -file=filename -- file for saving or loading pattern, in case of saving if not specified stdin is used
    <group> -- Group 1-4
    <pattern><a|b> - Pattern 1-8 A-B

Example -- view group 1 pattern 1A:
    td3pattern 1 1A
Example -- using loopback drivers save group 4 pattern 2B to file
    td3pattern -in=\"Loopback in 1\" -out=\"Loopback out 1\" 1 2B -file=pattern1-2B.txt
Example -- load file and upload it to group 3 pattern 8A
    td3pattern upload 1 1A -file=confusion-pattern.txt
";

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("{}\n{}", err, USAGE),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = config::get_config()?;
    let (out_midi, out_port, in_midi, in_port) = open_ports(&(config.out_port), &(config.in_port))?;
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    // midi receive thread is opened by midir
    let _in_connection = in_midi.connect(
        &in_port,
        "midir-read-input",
        move |_stamp, msg, _| {
            let _x = tx.send(msg.to_owned());
        },
        (),
    );
    // main/midi send
    let mut out_conn = out_midi.connect(&out_port, "").unwrap();
    match main_processor(&mut out_conn, rx, &config) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("Error has occured: {}", e);
            std::process::exit(1);
        }
    };
    /* loop {
        std::thread::yield_now();
    } */
}

fn main_processor(
    out_conn: &mut midir::MidiOutputConnection,
    rx: std::sync::mpsc::Receiver<std::vec::Vec<u8>>,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    let product_name_sysex = send_sysex(out_conn, &rx, "product name", &[0x06])?;
    // could also be .trim_matches(char::from(0)) for cutting traling 0
    let product_name = std::str::from_utf8(&product_name_sysex[1..product_name_sysex.len() - 1])?;
    if product_name != DEFAULT_PORTNAME {
        return Err(format!("Product name is: '{:?}', expected '{:?}'", product_name, DEFAULT_PORTNAME).into());
    }
    let fw_version_sysex = send_sysex(out_conn, &rx, "firmware version", &[0x08, 0x00])?;
    let fw_version = &fw_version_sysex[2..].iter().map(|x| format!("{}.", x)).collect::<String>();
    println!("Product Name {}, Firmware version is is {}", product_name, fw_version);

    let ab = if config.ab == 0 { "A" } else { "B" };
    match config.mode {
        Mode::Download => {
            let pattern_sysex = get_pattern(out_conn, &rx, config.group, config.pnum, config.ab)?;
            let pattern = sysex_to_pattern(&pattern_sysex);
            let string_pattern = pattern_to_string(&pattern);
            if config.filename.len() == 0 {
                println!("Group: {} Pattern: {}{}", config.group, config.pnum, ab);
                print!("\n{}", string_pattern);
            } else {
                let mut file = File::create(&config.filename)?;
                file.write_all(string_pattern.as_bytes())?;
                println!("Group {} Pattern: {}{} is saved to {}", config.group, config.pnum, ab, config.filename);
            }
        }
        Mode::Upload => {
            let string_pattern = read_to_string(&config.filename)?;
            let pattern = string_to_pattern(string_pattern)?;
            let pattern_sysex = pattern_to_sysex(&pattern, config.group, config.pnum, config.ab);
            let _ret = send_sysex(out_conn, &rx, "pattern", pattern_sysex.as_slice());
            // process _ret???
            println!("File {} is uploaded to Group {} Pattern: {}{}", config.filename, config.group, config.pnum, ab);
        }
    }
    Ok(())
}
