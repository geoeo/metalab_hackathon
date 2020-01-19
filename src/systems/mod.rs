use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::model::{commands::InputCommand, commands::Command, state::{State, Position}, commands::Move, EMPTY_ICON, ADD_ICON, SUBTRACT_ICON, MULT_ICON};
use crate::map::Grid;

pub fn simple_move(move_command: &Command, multiplier: i16) -> Move {
    match move_command {
        Command::Left => (-multiplier, 0),
        Command::Right => (multiplier, 0),
        Command::Up => (0, -multiplier),
        Command::Down => (0, multiplier),
        _ => (0, 0)
    }
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

pub fn process_input(input_command: &InputCommand) -> Command {
    if input_command.valid {
        match input_command.key_event.code {
            KeyCode::Left => Command::Left,
            KeyCode::Right => Command::Right,
            KeyCode::Up => Command::Up,
            KeyCode::Down => Command::Down,
            KeyCode::Char('a') => Command::Left,
            KeyCode::Char('d') => Command::Right,
            KeyCode::Char('w') => Command::Up,
            KeyCode::Char('s') => Command::Down,
            KeyCode::Char('j') => Command::PlaceAdd,
            KeyCode::Char('k') => Command::PlaceSubtract,
            KeyCode::Char('l') => Command::PlaceMultiplication,
            _ => Command::None
        }
    } else {
        Command::None
    }
}

pub fn modify_level(level: &mut Grid, cursor_position: &Position, command: Command) -> () {

    let current_char = match level.map.get(cursor_position.y_pos as usize,cursor_position.x_pos as usize) {
        Some(tile) => tile.icon,
        None => EMPTY_ICON
    };


    let new_char = match command {
        Command::PlaceAdd => {
            if current_char != ADD_ICON {
                ADD_ICON
            } else {
                EMPTY_ICON
            }
        },
        Command::PlaceSubtract => {
            if current_char != SUBTRACT_ICON {
                SUBTRACT_ICON
            } else {
                EMPTY_ICON
            }
        },
        Command::PlaceMultiplication => {
            if current_char != MULT_ICON {
                MULT_ICON
            } else {
                EMPTY_ICON
            }
        },
        _ => current_char
    };

    match level.map.get_mut(cursor_position.y_pos as usize,cursor_position.x_pos as usize) {
        Some(tile) => tile.icon = new_char,
        _ => ()
    }

}