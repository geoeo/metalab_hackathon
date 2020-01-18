use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::model::{commands::InputCommand, commands::MoveCommand,state::State, commands::Move};


pub fn simple_command(move_command: &MoveCommand, multiplier: i16) -> Move {
    match move_command {
        MoveCommand::Left => (-multiplier, 0),
        MoveCommand::Right => (multiplier, 0),
        MoveCommand::Up => (0, -multiplier),
        MoveCommand::Down => (0, multiplier),
        MoveCommand::None => (0, 0)
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

pub fn process_input(input_command: &InputCommand) -> MoveCommand {
    if input_command.valid {
        match input_command.key_event.code {
            KeyCode::Left => MoveCommand::Left,
            KeyCode::Right => MoveCommand::Right,
            KeyCode::Up => MoveCommand::Up,
            KeyCode::Down => MoveCommand::Down,
            KeyCode::Char('a') => MoveCommand::Left,
            KeyCode::Char('d') => MoveCommand::Right,
            KeyCode::Char('w') => MoveCommand::Up,
            KeyCode::Char('s') => MoveCommand::Down,
            _ => MoveCommand::None
        }
    } else {
        MoveCommand::None
    }
}