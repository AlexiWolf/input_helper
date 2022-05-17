use std::collections::HashMap;

use crate::*;

/// Represents the current state of an input for the [InputMap].
#[derive(Debug, Clone, Copy)]
pub enum InputState {
    Key(ButtonState),
}

impl ButtonState {
    pub fn is_pressed(&self) -> bool {
        match self {
            ButtonState::Up => false,
            ButtonState::Down => true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Input {
    Key(Key),
}

/// Provide a map for querying the current input state.
pub struct InputMap {
    map: HashMap<Input, InputState>,
    input_reader: Arc<Reader<InputEvent>>,
}

impl InputMap {
    pub fn new(input_helper: &InputHelper) -> Self {
        Self {
            map: HashMap::new(),
            input_reader: input_helper.reader(),
        }
    }

    /// Process all queued [InputEvent]s and update the map.
    ///
    /// This method should be ran before inspecting the input state.  If you're
    /// writing a framework using Input Helper, then you should probably run this before
    /// running the user's `update` code.
    pub fn update(&mut self) {
        for event in self.input_reader.read() {
            match event {
                InputEvent::Key(key, state) => {
                    self.map.insert(Input::Key(key), InputState::Key(state));
                }
            }
        }
    }

    /// Query the state of a button, and return `true` if the button is pressed.
    ///
    /// If the requested input is not a button, then `false` will be returned.
    pub fn is_pressed<I: Into<Input>>(&self, input: I) -> bool {
        match self.map.get(&input.into()) {
            Some(input) => match input {
                InputState::Key(button) => button.is_pressed(),
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod input_map_tests {
    use super::*;

    #[test]
    fn should_update_input_map() {
        let input_helper = InputHelper::new();
        let mut input_map = InputMap::new(&input_helper);

        input_helper.send(InputEvent::Key(Key::A, ButtonState::Down));
        input_map.update();
        assert!(input_map.is_pressed(Key::A));

        input_helper.send(InputEvent::Key(Key::A, ButtonState::Up));
        input_map.update();
        assert!(!input_map.is_pressed(Key::A));
    }
}
