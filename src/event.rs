use crate::*; 

/// Provides a set of generic input events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Key(Key, ButtonState),
}

impl InputEvent {
    pub fn key_up(key: Key) -> Self {
        Self::Key(key, ButtonState::Up)
    }
}

#[cfg(test)]
mod test_input_event {
    use super::*;

    #[test]
    fn should_have_key_up_method() {
        assert_eq!(InputEvent::Key(Key::A, ButtonState::Up), InputEvent::key_up(Key::A));
    }

    #[test]
    fn should_have_key_down_method() {
        assert_eq!(InputEvent::Key(Key::A, ButtonState::Down), InputEvent::key_down(Key::A));
    }
}
