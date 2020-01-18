pub mod tree;
pub mod model;
pub mod map;
pub mod systems;

use crate::tree::leaf;
use crate::model::{state::{State,Position}, commands::InputCommand};
use crate::map::Level;
use std::io::Write;
use std::time::Duration;
use crossterm::{
    event::{poll, Event, KeyEvent, KeyCode,KeyModifiers, read},
    execute, queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    Result,
    cursor,
};

pub fn run<W>(output: &mut W, level: &mut Level) -> Result<()> where W: Write{
    let mut running = State::Running;
    let mut cursor_pos = Position::new();
    execute!(output, terminal::EnterAlternateScreen)?;
    enable_raw_mode()?;
    while running == State::Running {
        queue!(
            output,
            style::ResetColor,
            terminal::Clear(ClearType::All)
        )?;

        match poll(Duration::from_millis(1000)) {
            Ok(true) => {
                let read = read()?;
                let (new_state, input_command_new) = systems::parse_input_event(&read);
                running = new_state;

                let input_command = systems::process_input(&input_command_new);
                let delta_move = systems::simple_move(&input_command, 1);

                cursor_pos = Position::apply_delta(&cursor_pos, delta_move.0, delta_move.1, level.width, level.height);
                systems::modify_level(level, &cursor_pos, input_command);

                Level::draw(output,level);
                queue!(output,cursor::MoveTo(cursor_pos.x_pos, cursor_pos.y_pos))?;
            }

            Ok(false) => {
                Level::draw(output,level);
                queue!(output,cursor::MoveTo(cursor_pos.x_pos, cursor_pos.y_pos))?;
            }

            Err(e) => {
                queue!(output, style::Print(e.to_string()), cursor::MoveToNextLine(1))?;
            }
        }

        output.flush()?;
    }

    execute!(
        output,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    disable_raw_mode()
}
