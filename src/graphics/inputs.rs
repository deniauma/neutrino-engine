use glutin::{KeyboardInput, ElementState, MouseScrollDelta};
pub use glutin::VirtualKeyCode as Key;
use std::collections::HashMap;


#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    LEFT,
    RIGHT,
    MIDDLE
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone)]
pub struct Input<'a> {
    keys: &'a HashMap<Key, ButtonState>,
    mouse_button_left: ButtonState,
    mouse_button_right: ButtonState,
    mouse_button_middle: ButtonState,
    mouse_motion_delta_x: f64,
    mouse_motion_delta_y: f64,
}

pub struct InputSystem {
    keys_state: HashMap<Key, ButtonState>,
    mouse_left: ButtonState,
    mouse_right: ButtonState,
    mouse_middle: ButtonState,
    mouse_x_delta: f64,
    mouse_y_delta: f64,
}

impl InputSystem {
    pub fn new() -> Self {
        Self {
            keys_state: HashMap::new(),
            mouse_left: ButtonState::RELEASED,
            mouse_right: ButtonState::RELEASED,
            mouse_middle: ButtonState::RELEASED,
            mouse_x_delta: 0.0,
            mouse_y_delta: 0.0,
        }
    }

    pub fn set_key_event(&mut self, event: KeyboardInput) {
        match event.virtual_keycode {
            Some(key) => {
                match event.state {
                    ElementState::Pressed => self.keys_state.insert(key, ButtonState::PRESSED),
                    ElementState::Released => self.keys_state.insert(key, ButtonState::RELEASED),
                };
                match key {
                    Key::Z => println!("Z key event => {:?}", self.keys_state[&Key::Z]),
                    _ => println!("key event: {:?}", key),
                }
            },
            None => (),
        }
    }

    pub fn set_mouse_button_event(&mut self, button: glutin::MouseButton, state: ElementState) {
        //println!("Mouse button: {:?}", button);
        match button {
            glutin::MouseButton::Left => match state {
                ElementState::Pressed => self.mouse_left.pressed(),
                ElementState::Released => self.mouse_left.released(),
            },
            glutin::MouseButton::Right => match state {
                ElementState::Pressed => self.mouse_right.pressed(),
                ElementState::Released => self.mouse_right.released(),
            },
            glutin::MouseButton::Middle => match state {
                ElementState::Pressed => self.mouse_middle.pressed(),
                ElementState::Released => self.mouse_middle.released(),
            },
            _ => (),
        }
    }

    pub fn set_mouse_move_event(&mut self, x_delta: f64, y_delta: f64) {
        //println!("Mouse cursor: ({}, {})", x_delta, y_delta);
        self.mouse_x_delta = x_delta;
        self.mouse_y_delta = y_delta;
    }

    pub fn is_key_pressed(&self, key: Key) -> ButtonState {
        match self.keys_state.get(&key) {
            Some(state) => *state,
            None => ButtonState::RELEASED,
        }
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> ButtonState {
        match button {
            MouseButton::LEFT => self.mouse_left,
            MouseButton::RIGHT => self.mouse_right,
            MouseButton::MIDDLE => self.mouse_middle,
        }
    }

    pub fn get_mouse_motion(&self) -> (f64, f64) {
        (self.mouse_x_delta, self.mouse_y_delta)
    }

    pub fn get_input(&self) -> Input {
        Input {
            keys: &self.keys_state,
            mouse_button_left: self.mouse_left,
            mouse_button_right: self.mouse_right,
            mouse_button_middle: self.mouse_middle,
            mouse_motion_delta_x: self.mouse_x_delta,
            mouse_motion_delta_y: self.mouse_y_delta,
        }
    }
}
