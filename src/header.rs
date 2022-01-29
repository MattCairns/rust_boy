#[derive(Debug)]
pub enum DestinationCode {
    Japanese,
    NonJapanese,
    Unknown(u8),
}

#[derive(Debug)]
pub enum CartridgeType {
    RomOnly,
    MBC1,
    MBC2,
    RomRam,
    MMM01,
    MBC3,
    MBC5,
    MBC6,
    MBC7,
    Unknown,
}

pub fn parse_cartridge_type(value: u8) -> CartridgeType {
    match value {
        0x00 => CartridgeType::RomOnly,
        0x01..=0x03 => CartridgeType::MBC1,
        0x05..=0x06 => CartridgeType::MBC2,
        0x08..=0x09 => CartridgeType::RomRam,
        0x0B..=0x0D => CartridgeType::MMM01,
        0x0F..=0x13 => CartridgeType::MBC3,
        0x19..=0x1E => CartridgeType::MBC5,
        0x20 => CartridgeType::MBC6,
        0x22 => CartridgeType::MBC7,
        _ => CartridgeType::Unknown,
    }
}
pub struct Header {
    pub logo: Vec<u8>,
    pub title: String,
    pub new_licensee_code: u16,
    pub old_licensee_code: u8,
    pub sgb_flag: bool,
    pub cartridge_type: CartridgeType,
    pub rom_size: u8,
    pub ram_size: u8,
    pub destination_code: DestinationCode, // Should be enum
    pub rom_version_num: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

impl Header {
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

        Header {
            logo,
            title,
            new_licensee_code: 0, // TODO
            old_licensee_code: 0, // TODO
            sgb_flag: false,
            cartridge_type: parse_cartridge_type(data[0x0147]),
            rom_size: data[0x0148], // Make this an enum?
            ram_size: data[0x0149], // Make this an enum?
            destination_code,
            rom_version_num: data[0x014C],
            header_checksum: data[0x014D],
            global_checksum: 0, // TODO
        }
    }

    pub fn is_compatible(&self) -> bool {
        match self.cartridge_type {
            CartridgeType::RomOnly => true,
            _ => false,
        }
    }

    pub fn print_logo(&self) {
        let mut logo_str = String::from("");
        (0..8).for_each(|y| {
            let mut i = ((y / 2) % 2) + (y / 4) * 24;
            (0..12).for_each(|_| {
                let nibble: u8 = if (y % 2) == 0 {
                    self.logo[i] >> 4
                } else {
                    self.logo[i] & 0xF
                };

                (0..4).rev().for_each(|b| {
                    if (nibble >> b) & 1 == 0 {
                        logo_str.push(' ');
                    } else {
                        logo_str.push('▓');
                    }
                });

                i += 2;
            });

            logo_str.push('\n');
        });
        println!("{}", logo_str);
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}\nDestination Code: {:?}\nCartridge Type: {:?}\nROM Size: {}\nRAM Size: {}\nROM Ver: {}\nHeader Checksum: {}\nLogo: {:X?}",
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
