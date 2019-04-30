use crate::graphics::{Index, ComponentStorageManager};
use crate::graphics::inputs::InputSystem;
use std::collections::HashMap;


pub trait SceneUpdate {
    fn update(&self, data: &mut ComponentStorageManager);
}

pub trait EntityState {
    fn on_create(&mut self, data: &mut ComponentStorageManager);
    fn on_update(&mut self, data: &mut ComponentStorageManager, input: &InputSystem, delta: f32);
    fn on_delete(&mut self, data: &mut ComponentStorageManager);
}

pub struct StateSystem {
    states_manager: HashMap<Index, Box<EntityState>>
}

impl StateSystem {
    pub fn new() -> Self {
        Self {
            states_manager: HashMap::new()
        }
    }

    pub fn set_entity_states<T: EntityState>(&mut self, id: Index, states: T)
    where
        T: 'static,
    {
        self.states_manager.insert(id, Box::new(states));
    }

    pub fn run_update_state(&mut self, data: &mut ComponentStorageManager, input: &InputSystem, delta: f32) {
        for (_, states) in self.states_manager.iter_mut() {
            states.on_update(data, input, delta);
        }
    }
}