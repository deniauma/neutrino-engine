use glutin::{KeyboardInput, VirtualKeyCode, ElementState};
use std::collections::HashMap;

pub type Key = VirtualKeyCode;

pub struct InputSystem {
    keys_status: HashMap<Key, bool>,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            keys_status: HashMap::new(),
        }
    }

    pub fn get_key_event(&mut self, event: KeyboardInput) {
        match event.virtual_keycode {
            Some(key) => {
                match event.state {
                    ElementState::Pressed => self.keys_status.insert(key, true),
                    ElementState::Released => self.keys_status.insert(key, false),
                };
                match key {
                    VirtualKeyCode::Z => println!("Z key event => {:?}", self.keys_status[&VirtualKeyCode::Z]),
                    _ => println!("key event: {:?}", key),
                }
            },
            None => (),
        }
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_status[&key]
    }
}