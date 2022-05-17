//! The main job of Input Helper is to provide a single, unified way to process input
//! from any input framework.
//!
//! # The Problem
//!
//! Most libraries that take user input do so in a library-specific way.  In order to
//! use the library, you must write library-specific code.
//!
//! For example: You can't switch from Winit to SDL, or Gilrs to SDL, or any other
//! library without having to rewrite all your input code. This can either couple your
//! project to specific framework(s), or force you to waste time duplicating code.  In the
//! case of using multiple input libraries, such as pairing Winit and Gilrs you must
//! handle each framework separately.
//!
//! # The Solution
//!
//! Library-specific input events can be converted to [InputEvent]s, and sent through
//! Input Helper using [InputHelper::send()].  Then your application code can be written
//! against Input Helper.  If you ever need to switch your input system, then there's no
//! need to rewrite your business logic.  This also allows you to process all input
//! in a single place regardless of its source.
//!
//! Input Helper will provide default integration functions for common frameworks such as
//! Winit, SDL, and Gilrs.  
//!
//! # Getting Started
//!
//! Create the [InputHelper] and send it some [InputEvent] events.
//!
//! ```
//! use input_helper::*;
//!
//! let input_helper = InputHelper::new();
//!
//! input_helper.send(InputEvent::key_up(Key::A));
//! input_helper.send(InputEvent::key_down(Key::A));
//! ```
//!
//! Read events using an [InputReader].  You can request a new [InputReader] using
//! [InputHelper::reader()].
//!
//! ```
//! # use input_helper::*;
//! #
//! # let input_helper = InputHelper::new();
//! #
//! let input_reader = input_helper.reader();
//!
//! for input in input_reader.read() {
//!     // Do something cool.
//! }
//! ```

mod button;
mod event;
mod keyboard;

pub use button::*;
pub use event::*;
pub use keyboard::*;

use std::sync::{Arc, Mutex};

use event_feed::*;

/// Defines a [Feed] of [InputEvent]s.
pub type InputFeed = Feed<InputEvent>;

/// Defines a [Reader] for [InputEvent]s.
pub type InputReader = Reader<InputEvent>;

/// Provides a generic input event queue.
pub struct InputHelper {
    events: Arc<Mutex<Feed<InputEvent>>>,
}

impl InputHelper {
    pub fn new() -> Self {
        Self {
            events: Arc::from(Mutex::from(Feed::new())),
        }
    }

    /// Create a new [InputReader].
    pub fn reader(&self) -> Arc<Reader<InputEvent>> {
        self.events
            .lock()
            .expect("failed to unlock the event feed")
            .add_reader()
    }

    /// Send an [InputEvent] through the input helper.
    pub fn send(&self, input: InputEvent) {
        self.events
            .lock()
            .expect("failed to unlock the event feed")
            .send(input);
    }
}

impl Default for InputHelper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod input_helper_tests {
    use super::*;

    #[test]
    fn should_read_keys() {
        let input_helper = InputHelper::new();
        let input_reader = input_helper.reader();

        input_helper.send(InputEvent::Key(Key::A, ButtonState::Down));
        input_helper.send(InputEvent::Key(Key::A, ButtonState::Up));

        assert_eq!(input_reader.read().count(), 2);
    }
}
