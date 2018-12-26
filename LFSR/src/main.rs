struct LFSR16 {
    seed: u16,
    lfsr: u16,
    bit: u16,
}

impl Default for LFSR16 {
    fn default() -> LFSR16 {
        LFSR16 {
            seed: 0xCA75u16,  // Hard-coded from stream given by reviewer
            lfsr: 0xCA75u16,  // Initial state
            bit: 0
        }
    }
}

impl Iterator for LFSR16 {
    type Item = u16;
    
    fn next(&mut self) -> Option<u16> {
        // Taps: 16, 12, 3, 1 -- hard-coded from BM algorithm
        self.bit = ((self.lfsr >> 0) ^ (self.lfsr >> 3) ^ (self.lfsr >> 12) ^
                        (self.lfsr >> 14)) & 1;
        self.lfsr = (self.lfsr >> 1) | (self.bit << 15);

        if self.lfsr != self.seed {
            Some(self.lfsr)
        } else {
            None
        }
    }
}


fn main() {
    let lfsr = LFSR16::default();

    for i in lfsr {
        println!("{:#018b}", i);
    }
}
