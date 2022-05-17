/// Represents a button which can be pressed, or not pressed.
pub trait Button {
    /// Returns true if the button is pressed.
    fn is_pressed(&self) -> bool;
}

/// Provides state for a button, indicating if it is currently up or down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Up,
    Down,
}

impl Button for ButtonState {
    fn is_pressed(&self) -> bool {
        match self {
            ButtonState::Up => false,
            ButtonState::Down => true,
        }
    }
}

