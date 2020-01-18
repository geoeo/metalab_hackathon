use crate::tree::leaf;
use std::io::Write;
use std::time::Duration;
use crossterm::{
    event::{poll, read},
    execute, queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    Result,
    cursor,
};

pub mod tree;
pub mod map;


use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers}
};

#[derive(Debug,Copy,Clone)]
pub struct InputCommand {
    pub valid: bool,
    pub key_event: KeyEvent
}

#[repr(u8)]
#[derive(Copy,Clone,PartialEq)]
pub enum State {
    Running,
    Pause,
    Quit,
}

pub fn parse_input_event(event: &Event) -> (State, InputCommand) {

    match event {
        Event::Key(keyboard) => {
            (match keyboard.code {
                KeyCode::Esc => State::Quit,
                _ => State::Running
            }, InputCommand {valid: true, key_event: keyboard.clone()})
        },
        _ => (State::Running, InputCommand {valid: false, key_event: KeyEvent{code: KeyCode::Esc, modifiers: KeyModifiers::empty()}})
    }

}

pub fn run<W>(output: &mut W) -> Result<()> where W: Write{
    let mut running = State::Running;
    execute!(output, terminal::EnterAlternateScreen)?;
    enable_raw_mode()?;
    while running == State::Running {
        queue!(
            output,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        match poll(Duration::from_millis(1000)) {
            Ok(true) => {
                let read = read()?;
                let (new_state, input_command_new) = parse_input_event(&read);
                running = new_state;
                queue!(output, style::Print("running"), cursor::MoveToNextLine(1))?;
            }

            Ok(false) => {
                queue!(output, style::Print("no input detected"), cursor::MoveToNextLine(1))?;
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
