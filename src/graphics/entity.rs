use crate::graphics::{Mesh, Material, MaterialBuilder, Transform, PrimitiveBuilder, Engine};

#[derive(Debug)]
pub struct EntityBuilder {
    mesh: Mesh,
    material: Material,
    transform: Transform
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            mesh: Mesh::new_empty(),
            material: MaterialBuilder::simple_material_2d(),
            transform: Transform::new_default(),
        }
    }

    pub fn with_mesh(&mut self, mesh: Mesh) -> &mut Self {
        self.mesh = mesh;
        self
    }

    pub fn with_material(&mut self, material: Material) -> &mut Self {
        self.material = material;
        self
    }

    pub fn with_texture(&mut self, texture_path: &str) -> &mut Self {
        self.material = MaterialBuilder::simple_texture_material_2d(texture_path);
        self
    }

    pub fn with_transform(&mut self, transform: Transform) -> &mut Self {
        self.transform = transform;
        self
    }

    pub fn with_quad_mesh(&mut self, size: f32) -> &mut Self {
        self.mesh = PrimitiveBuilder::quad();
        self.transform.scale *= size;
        self
    }

    pub fn with_cube_mesh(&mut self, size: f32) -> &mut Self {
        self.mesh = PrimitiveBuilder::cube();
        self.transform.scale *= size;
        self
    }

    pub fn build(&self, engine: &mut Engine) -> u32 {
        let entity = engine.create_scene_object();
        engine.add_mesh(entity.id, self.mesh.clone());
        engine.add_material(entity.id, self.material);
        engine.add_transform(entity.id, self.transform);
        entity.id
    }
}