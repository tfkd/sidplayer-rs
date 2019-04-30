use std::env;
use std::fs::File;
use std::io::prelude::*;
use byteorder::{ByteOrder, BigEndian};
use resid::{ChipModel, Sid};
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = match self.phase {
                v if v >= 0.0 && v < 0.5 => self.volume,
                _ => - self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

#[derive(Debug)]
struct Header {
    magic_id: String,
    version: u16,
    data_offset: u16,
    load_address: u16,
    init_address: u16,
    play_address: u16,
    songs: u16,
    start_song: u16,
    speed: u32,
    name: String,
    author: String,
    released: String,
}

#[derive(Debug)]
struct AdditionalHeader {
    flags: u16,
    start_page: u8,
    page_length: u8,
    second_sid_address: u8,
    third_sid_address: u8,
}

static SID_DATA: [u16; 51] = [
    25, 177, 250, 28, 214, 250,
    25, 177, 250, 25, 177, 250,
    25, 177, 125, 28, 214, 125,
    32, 94, 750, 25, 177, 250,
    28, 214, 250, 19, 63, 250,
    19, 63, 250, 19, 63, 250,
    21, 154, 63, 24, 63, 63,
    25, 177, 250, 24, 63, 125,
    19, 63, 250,
];

fn main() -> std::io::Result<()> {
    if let Some(file_name) = env::args().nth(1) {
        println!("File name: {}", file_name);

        let mut f = File::open(file_name)?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        println!("{:b}", buffer[0]);
        println!("{:x}", buffer[0]);

        let header = Header {
            magic_id: String::from_utf8(buffer[0..4].to_vec()).unwrap(),
            version: BigEndian::read_u16(&buffer[4..6]),
            data_offset: BigEndian::read_u16(&buffer[6..8]),
            load_address: BigEndian::read_u16(&buffer[8..10]),
            init_address: BigEndian::read_u16(&buffer[10..12]),
            play_address: BigEndian::read_u16(&buffer[12..14]),
            songs: BigEndian::read_u16(&buffer[14..16]),
            start_song: BigEndian::read_u16(&buffer[16..18]),
            speed: BigEndian::read_u32(&buffer[18..22]),
            name: String::from_utf8(buffer[22..54].to_vec()).unwrap(),
            author: String::from_utf8(buffer[54..86].to_vec()).unwrap(),
            released: String::from_utf8(buffer[86..118].to_vec()).unwrap(),
        };
        println!("{:?}", header);

        if header.version > 1 {
            let header2 = AdditionalHeader {
                flags: BigEndian::read_u16(&buffer[118..134]),
                start_page: buffer[134],
                page_length: buffer[135],
                second_sid_address: buffer[136],
                third_sid_address: buffer[137],
            };
            println!("{:?}", header2)
        }

        let mut sid = Sid::new(ChipModel::Mos6581);
        sid.write(0x05, 0x09);
        sid.write(0x06, 0x00);
        sid.write(0x06, 0x0f);
        let mut i = 0;
        while i < SID_DATA.len() {
            sid.write(0x01, SID_DATA[i + 0] as u8);
            sid.write(0x00, SID_DATA[i + 1] as u8);
            sid.write(0x00, 0x21);
            for _j in 0..SID_DATA[i + 2] {
                sid.clock_delta(22);
            }
            sid.write(0x00, 0x20);
            for _j in 0..50 {
                sid.clock_delta(22);
            }
            i += 3;
        }
        println!("{:?}", sid.read_state());
    }

    // sdl2 audio
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None
    };
    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25
        }
    }).unwrap();
    device.resume();
    std::thread::sleep(Duration::from_millis(2000));
    Ok(())
}
