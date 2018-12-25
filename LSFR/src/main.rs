struct LFSR16 {
    seed: u16,
    lfsr: u16,
    bit: u16,
}

impl Default for LFSR16 {
    fn default() -> LFSR16 {
        LFSR16 {
            seed: 0b1100101001110101,
            lfsr: 0b1100101001110101,
            bit: 0
        }
    }
}

impl Iterator for LFSR16 {
    type Item = u16;
    
    fn next(&mut self) -> Option<u16> {
        // Taps: 10, 8
        None
    }
}


fn main() {
    let mut lfsr = LFSR16::default();

    for _ in 0..100 {
        println!("{:?}", &lfsr.next());
    }
}
