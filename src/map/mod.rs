pub mod tile;

use array2d::Array2D;
use crate::map::tile::Tile;
use std::io::Write;
use crossterm::{queue,Result, cursor, style};
use crate::model::ERROR_ICON;

#[derive(Debug,Clone)]
pub struct Level {
    pub width : u16,
    pub height: u16,
    pub map: Array2D<Tile>
}

impl Level {
    pub fn new(width: u16, height: u16) -> Level {
        Level{
            width: width,
            height: height,
            map: Array2D::filled_with(Tile::new(),height as usize,width as usize)
        }

    }
    pub fn draw<W>(output: &mut W, level: &Level) -> Result<()> where W:Write {

        let x_offset = 0;
        let y_offset = 0;

        for x in 0..level.width {
            for y in 0..level.height {
                let x_term_pos = x as u16 +x_offset;
                let y_term_pos = y as u16 +y_offset;
                let element = level.map.get(y as usize,x as usize);
                match element {
                    Some(tile) => queue!(
                                    output,
                                    cursor::MoveTo(x_term_pos, y_term_pos),
                                    style::Print(tile.icon),
                                    ),
                    None => queue!(
                    output,
                    cursor::MoveTo(x_term_pos, y_term_pos),
                    style::Print(ERROR_ICON),
                    )
                }?

            }
        }

        queue!(
            output,
            cursor::MoveTo(0,level.height as u16 + 1)
    )


    }
}