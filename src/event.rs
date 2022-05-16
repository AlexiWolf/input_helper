use crate::*; 

/// Provides a set of generic input events.
#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    Key(Key, ButtonState),
}
