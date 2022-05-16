use crate::*; 

/// Provides a set of generic input events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Key(Key, ButtonState),
}

#[cfg(test)]
mod test_input_event {
    use super::*;

    #[test]
    fn should_have_key_up_method() {
        assert_eq!(InputEvent::Key(Key::A, ButtonState::Up), InputEvent::key_up(Key::A));
    }
}
