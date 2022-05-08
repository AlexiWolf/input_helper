use std::sync::{Arc, Mutex}; 
use std::collections::HashMap;

use event_feed::*;

pub type InputFeed = Feed<InputEvent>;
pub type InputReader = Reader<InputEvent>;
pub type InputName = &'static str;

#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    KeyDown(InputName),
    KeyUp(InputName),
    // ext. ext.
}

pub enum InputState {
    Key(ButtonState),
}


pub enum ButtonState {
    Up,
    Down,
}

impl ButtonState {
    pub fn is_pressed(&self) -> bool {
        match self {
            ButtonState::Up => false,
            ButtonState::Down => true,
        }
    }
}

pub struct InputHelper {
    events: Arc<Mutex<Feed<InputEvent>>>,
}

impl InputHelper {
    pub fn new() -> Self {
        Self {
            events: Arc::from(Mutex::from(Feed::new())),
        }
    }

    pub fn reader(&self) -> Arc<Reader<InputEvent>> {
        self.events.lock().expect("failed to unlock the event feed").add_reader()
    }

    pub fn send(&self, input: InputEvent) {
        self.events.lock().expect("failed to unlock the event feed").send(input);
    }
}

impl Default for InputHelper {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InputMap {
    map: HashMap<InputName, InputState>,
    input_reader: Arc<Reader<InputEvent>>,
}

impl InputMap {
    pub fn new(input_helper: &InputHelper) -> Self {
        Self {
            map: HashMap::new(),
            input_reader: input_helper.reader(),
        }
    }

    pub fn update(&mut self) {
        for event in self.input_reader.read() {
            match event {
                InputEvent::KeyDown(key) => {
                    self.map.insert(key, InputState::Key(ButtonState::Down));
                }
                InputEvent::KeyUp(key) => {
                    self.map.insert(key, InputState::Key(ButtonState::Up));
                }
            }
        }
    }

    pub fn is_pressed(&self, input: InputName) -> bool {
        match self.map.get(input) {
            Some(input) => match input {
                InputState::Key(button) => button.is_pressed(),
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod input_helper_tests {
    use super::*;

    #[test]
    fn should_read_keys() {
        let input_helper = InputHelper::new();
        let input_reader = input_helper.reader();

        input_helper.send(InputEvent::KeyDown("a"));
        input_helper.send(InputEvent::KeyDown("a"));

        assert_eq!(input_reader.read().count(), 2);
    }

    #[test]
    fn should_update_input_map() {
        let input_helper = InputHelper::new();
        let mut input_map = InputMap::new(&input_helper);

        input_helper.send(InputEvent::KeyDown("a"));
        input_map.update();
        assert!(input_map.is_pressed("a"));

        input_helper.send(InputEvent::KeyUp("a"));
        input_map.update();
        assert!(!input_map.is_pressed("a"));
    }
}
