use std::env;
use std::fs::File;
use std::io::prelude::*;
use byteorder::{ByteOrder, BigEndian};

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
    }
    Ok(())
}
