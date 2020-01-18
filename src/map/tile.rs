use crate::model::EMPTY_ICON;

#[derive(Debug, Clone)]
pub struct Tile {
    pub icon: char
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            icon: EMPTY_ICON
        }
    }
}