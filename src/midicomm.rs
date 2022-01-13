use std::error::Error;

pub fn open_ports(
    out_port: &String,
    in_port: &String,
) -> Result<(midir::MidiOutput, midir::MidiOutputPort, midir::MidiInput, midir::MidiInputPort), Box<dyn Error>> {
    // Out
    let out_midi = midir::MidiOutput::new("").expect("Failed create MidiOutput");
    let ports = out_midi.ports();
    let out_port = match ports.iter().find(|p| out_midi.port_name(p).unwrap() == *out_port) {
        Some(p) => p,
        None => {
            let port_names = ports.iter().map(|p| out_midi.port_name(p).expect("")).collect::<Vec<String>>().join(", ");
            return Err(format!("Output port \"{}\" is not found, available ports: {}", out_port, port_names).into());
        }
    };
    // In
    let mut in_midi = midir::MidiInput::new("").expect("Failed create MidiOutput");
    in_midi.ignore(midir::Ignore::TimeAndActiveSense);
    let ports = in_midi.ports();
    let in_port = match ports.iter().find(|p| in_midi.port_name(p).unwrap() == *in_port) {
        Some(p) => p,
        None => {
            let port_names = ports.iter().map(|p| in_midi.port_name(p).expect("")).collect::<Vec<String>>().join(", ");
            return Err(format!("Input port \"{}\" is not found, available ports: {}", in_port, port_names).into());
        }
    };
    Ok((out_midi, out_port.to_owned(), in_midi, in_port.to_owned()))
}

const SYX_PRE: &'static [u8] = &[0xF0, 0x00, 0x20, 0x32, 0x00, 0x01, 0x0A];
const SYX_POST: &'static [u8] = &[0xF7];

pub async fn send_sysex(
    out_conn: &mut midir::MidiOutputConnection,
    rx: &mut tokio::sync::mpsc::Receiver<std::vec::Vec<u8>>,
    desc: &str,
    data_smsg: &[u8],
) -> Result<std::vec::Vec<u8>, Box<dyn Error>> {
    if cfg!(debug_assertions) {
        println!(">> Requesting {}, data part of message = {:02x?}", desc, data_smsg);
    }
    out_conn.send(SYX_PRE).unwrap();
    out_conn.send(&data_smsg).unwrap();
    out_conn.send(SYX_POST).unwrap();
    let rmsg = rx.recv().await; // std::vec::Vec<u8>
    match rmsg {
        Some(m) => {
            if cfg!(debug_assertions) {
                println!("<< Response ({}b) {:02x?}", m.len(), m);
            }
            // Match head and tail
            if m.len() < SYX_PRE.len() + SYX_POST.len() + 1
                || (&SYX_PRE)[0..SYX_PRE.len()].iter().zip(&m[0..SYX_PRE.len()]).filter(|&(a, b)| a != b).count() > 0
                || *m.last().unwrap() != SYX_POST[0]
            {
                return Err(format!("Response for {} has wrong size", desc).into());
            } else {
                return Ok((&m[SYX_PRE.len()..m.len() - 1]).to_owned());
            }
        }
        None => Err(format!("No response for {} has been received", desc).into()),
    }
}

pub async fn get_pattern(
    out_conn: &mut midir::MidiOutputConnection,
    rx: &mut tokio::sync::mpsc::Receiver<std::vec::Vec<u8>>,
    group: u8,
    pnum: u8,
    ab: u8,
) -> Result<std::vec::Vec<u8>, Box<dyn Error>> {
    if group > 3 {
        return Err("Invalid group specified".into());
    } else if pnum > 7 {
        return Err("Invalid pattern specified".into());
    } else if ab > 1 {
        return Err("Invalid ab specified".into());
    }
    let desc = format!("Pattern Group {} Pattern {}{}", group + 1, pnum + 1, if ab == 0 { "a" } else { "b" });
    send_sysex(out_conn, rx, &desc, &[0x77, group, pnum + (ab << 3)]).await
}
