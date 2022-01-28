pub struct Tile {
    pub data: Vec<u8>,
}

impl Tile {
    pub fn new(data: Vec<u8>) -> Self {
        assert!(data.len() == 16);
        Self { data }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row = String::from("");

        for i in (0..16).step_by(2) {
            let mut bit_pos = 0;
            while bit_pos < 8 {
                let offset = bit_pos % 8;
                let hb = self.data[i] & (1 << offset) != 0;
                let lb = self.data[i + 1] & (1 << offset) != 0;

                if lb == false && hb == false {
                    row.push(' ');
                    row.push(' ');
                } else if lb == false && hb == true {
                    row.push('░');
                    row.push('░');
                } else if lb == true && hb == false {
                    row.push('▒');
                    row.push('▒');
                } else if lb == true && hb == true {
                    row.push('▓');
                    row.push('▓');
                }

                bit_pos += 1;
            }
            print!("{:2X} {:2X} ", self.data[i], self.data[i + 1]);
            row.push('\n');
        }

        println!("");
        write!(f, "{}", row)
    }
}
