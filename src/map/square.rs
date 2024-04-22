#[derive(Clone,Default)]
pub struct Square{
    pub location: (u32,u32),
    pub cursor: bool,
    pub is_cleaned: bool,
    pub is_portal: bool,
    pub is_wall: bool,
}

impl Square{
    pub fn new(x: u32, y: u32, cursor: bool,is_cleaned: bool, is_portal: bool, is_wall: bool) -> Self{
        Square{
            location: (x,y),
            cursor,
            is_cleaned,
            is_portal,
            is_wall
        }
    }
}