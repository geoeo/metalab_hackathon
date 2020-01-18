#[repr(u8)]
#[derive(Copy,Clone,PartialEq)]
pub enum State {
    Running,
    Pause,
    Quit,
}