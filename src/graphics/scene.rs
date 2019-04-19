use math::Mat4;
use crate::graphics::{Index, ComponentStorageManager};
use std::collections::HashMap;


struct SceneManager {
    transforms: HashMap<Index, Mat4>
}

impl SceneManager {
    pub fn update_transforms(&mut self, storage: &ComponentStorageManager) {
        for (id, trans) in storage.transform_manager.iter() {
            //self.transforms.insert(*id, trans.calculate_local_transform());
        }
    }
}