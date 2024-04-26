#[derive(Copy, Clone)]
pub struct Square {
    pub location: (usize, usize),
    pub cursor: bool,
    pub is_cleaned: bool,
    pub is_portal: bool,
    pub is_wall: bool,
}

impl Default for Square {
    fn default() -> Self {
        Square {
            location: (0, 0),
            cursor: false,
            is_cleaned: false,
            is_portal: false,
            is_wall: false,
        }
    }
}
