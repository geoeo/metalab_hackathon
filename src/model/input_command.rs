use crossterm::event::KeyEvent;

#[derive(Debug,Copy,Clone)]
pub struct InputCommand {
    pub valid: bool,
    pub key_event: KeyEvent
}