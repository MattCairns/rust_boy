use std::fs::File;
use std::io::prelude::*;

/* const CARTRIDGE_SIZE: usize = 0x200000;
const HEADER_SIZE: usize = 0x4F;
const HEADER_START: usize = 0x0100;
const HEADER_END: usize = 0x014F; */

#[derive(Debug)]
pub enum DestinationCode {
    Japanese,
    NonJapanese,
    Unknown(u8),
}

impl std::fmt::Display for DestinationCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DestinationCode::Japanese => write!(f, "Japanese"),
            DestinationCode::NonJapanese => write!(f, "Non-Japanese"),
            DestinationCode::Unknown(n) => write!(f, "{}", n),
        }
    }
}

pub struct CartridgeHeader {
    pub logo: Vec<u8>,
    pub title: String,
    pub new_licensee_code: u16,
    pub old_licensee_code: u8,
    pub sgb_flag: bool,
    pub cartridge_type: u8, // This should be an enum
    pub rom_size: u8,
    pub ram_size: u8,
    pub destination_code: DestinationCode, // Should be enum
    pub rom_version_num: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

impl CartridgeHeader {
    pub fn new(data: &Vec<u8>) -> Self {
        let logo = data[0x104..0x0133 + 0x1].to_vec();
        let title = match std::str::from_utf8(&data[0x0134..0x0143 + 0x1]) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8: {}", e),
        }
        .to_string();

        let destination_code = match data[0x014A] {
            0x00 => DestinationCode::Japanese,
            0x01 => DestinationCode::NonJapanese,
            _ => DestinationCode::Unknown(data[0x014A]),
        };

        CartridgeHeader {
            logo,
            title,
            new_licensee_code: 0, // TODO
            old_licensee_code: 0, // TODO
            sgb_flag: false,
            cartridge_type: data[0x0147],
            rom_size: data[0x0148], // Make this an enum?
            ram_size: data[0x0149], // Make this an enum?
            destination_code,
            rom_version_num: data[0x014C],
            header_checksum: data[0x014D],
            global_checksum: 0, // TODO
        }
    }

    pub fn print_info(&self) {
        println!("TITLE: {}", self.title);
        println!("Destination Code: {}", self.destination_code);
    }

    pub fn print_logo(&self) {
        let mut logo_str = String::from("");
        (0..8).for_each(|y| {
            let mut i = ((y / 2) % 2) + (y / 4) * 24;
            (0..12).for_each(|x| {
                let mut bit_pos = 0;
                while bit_pos < 8 {
                    let offset = bit_pos % 8;
                    let b = self.logo[i] & (1 << offset) != 0;

                    if b == false {
                        logo_str.push(' ');
                        logo_str.push(' ');
                    } else {
                        logo_str.push('▓');
                        logo_str.push('▓');
                    }

                    bit_pos += 1;
                }
                // print!("{:2X} {:2X} ", self.data[i], self.data[i + 1]);
                i += 2;
            });

            // logo_str.push('\n');
        });
        println!("{}", logo_str);
    }
}

impl std::fmt::Display for CartridgeHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}\nDestination Code: {}\nCartridge Type: {}\nROM Size: {}\nRAM Size: {}\nROM Ver: {}\nHeader Checksum: {}\nLogo:\n{:X?}",
            self.title,
            self.destination_code,
            self.cartridge_type,
            self.rom_size,
            self.ram_size,
            self.rom_version_num,
            self.header_checksum,
            self.logo
        )
    }
}

pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn load(path: &str) -> Self {
        let data = read_file_as_bytes(path).unwrap();
        /* let header: [u8; HEADER_SIZE] = data[HEADER_START..HEADER_END]
        .try_into()
        .expect("Slice is not corrent length"); */
        Self { data }
    }
}

pub fn read_file_as_bytes(path: &str) -> Result<Vec<u8>, &'static str> {
    let mut f = File::open(path).expect("Cartridge could not be opened");
    let mut cartridge = Vec::new();

    f.read_to_end(&mut cartridge)
        .expect("Could not read cartridge");

    Ok(cartridge)
}
