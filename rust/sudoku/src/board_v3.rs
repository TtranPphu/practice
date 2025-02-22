#[derive(Debug, Clone)]
pub struct Board {
    cells: [u8; 81],
    candidates: [u16; 27],
    unsets: u8,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [0; 81],
            candidates: [0x1FF; 27],
            unsets: 81,
        }
    }
}
