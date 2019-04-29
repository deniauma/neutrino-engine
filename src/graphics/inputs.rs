use glutin::{KeyboardInput, VirtualKeyCode, ElementState, MouseButton, MouseScrollDelta};
use std::collections::HashMap;

pub type Key = VirtualKeyCode;

#[derive(Debug, Copy, Clone)]
pub enum ButtonState {
    PRESSED,
    RELEASED
}

impl ButtonState {
    pub fn pressed(&mut self) {
        *self = ButtonState::PRESSED;
    }

    pub fn released(&mut self) {
        *self = ButtonState::RELEASED;
    }
}

pub struct InputSystem {
    keys_status: HashMap<Key, ButtonState>,
    mouse_left: ButtonState,
    mouse_right: ButtonState,
    mouse_middle: ButtonState,
    mouse_x_delta: f64,
    mouse_y_delta: f64,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            keys_status: HashMap::new(),
            mouse_left: ButtonState::RELEASED,
            mouse_right: ButtonState::RELEASED,
            mouse_middle: ButtonState::RELEASED,
            mouse_x_delta: 0.0,
            mouse_y_delta: 0.0,
        }
    }

    pub fn get_key_event(&mut self, event: KeyboardInput) {
        match event.virtual_keycode {
            Some(key) => {
                match event.state {
                    ElementState::Pressed => self.keys_status.insert(key, ButtonState::PRESSED),
                    ElementState::Released => self.keys_status.insert(key, ButtonState::RELEASED),
                };
                match key {
                    VirtualKeyCode::Z => println!("Z key event => {:?}", self.keys_status[&VirtualKeyCode::Z]),
                    _ => println!("key event: {:?}", key),
                }
            },
            None => (),
        }
    }

    pub fn get_mouse_button_event(&mut self, button: MouseButton, state: ElementState) {
        //println!("Mouse button: {:?}", button);
        match button {
            MouseButton::Left => match state {
                ElementState::Pressed => self.mouse_left.pressed(),
                ElementState::Released => self.mouse_left.released(),
            },
            MouseButton::Right => match state {
                ElementState::Pressed => self.mouse_right.pressed(),
                ElementState::Released => self.mouse_right.released(),
            },
            MouseButton::Middle => match state {
                ElementState::Pressed => self.mouse_middle.pressed(),
                ElementState::Released => self.mouse_middle.released(),
            },
            _ => (),
        }
    }

    pub fn get_mouse_move_event(&mut self, x_delta: f64, y_delta: f64) {
        //println!("Mouse cursor: ({}, {})", x_delta, y_delta);
        self.mouse_x_delta = x_delta;
        self.mouse_y_delta = y_delta;
    }

    pub fn is_key_pressed(&self, key: Key) -> ButtonState {
        self.keys_status[&key]
    }
}
