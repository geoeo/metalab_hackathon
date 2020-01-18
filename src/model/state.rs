#[repr(u8)]
#[derive(Copy,Clone,PartialEq)]
pub enum State {
    Running,
    Pause,
    Quit,
}

#[derive(Debug, Copy,Clone)]
pub struct Position {
    pub x_pos: u16,
    pub y_pos: u16
}

impl Position {

    pub fn new() -> Position {
        Position {
            x_pos: 0,
            y_pos: 0
        }
    }

    pub fn apply_delta(position: &Position, delta_x: i16, delta_y: i16, level_width: u16, level_height: u16) -> Position {
        let x_new = match position.x_pos as i16 + delta_x {
            x if x < 0 => 0,
            x if x >= level_width as i16 => level_width - 1,
            x => x as u16
        };

        let y_new = match position.y_pos as i16 + delta_y {
            y if y < 0 => 0,
            y if y >= level_width as i16 => level_height - 1,
            y => y as u16
        };

        Position {
            x_pos:x_new,
            y_pos:y_new
        }
    }

}
