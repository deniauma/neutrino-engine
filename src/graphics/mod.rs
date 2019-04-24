extern crate gl;
extern crate glutin;

use glutin::dpi::*;
use glutin::GlContext;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::time::{Duration, Instant};

pub mod mesh;
pub mod shader;
pub mod transform;
pub mod scene;
pub mod states;
pub mod primitives;
pub mod entity;

use self::transform::Transform;
use self::mesh::*;
use self::shader::*;
use self::scene::*;
use self::states::*;
use self::primitives::*;


pub type Index = u32;

pub struct SimpleState {
    pub update: Fn(),
}

#[derive(PartialEq, Eq, Hash)]
pub enum EngineComponent {
    TRANSFORM,
    MESH,
    SHADER,
    MATERIAL,
}

#[derive(Copy, Clone)]
struct RenderObject {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
}

impl RenderObject {
    fn new() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            ebo: 0,
        }
    }
}

struct RenderSystem {
    objects_to_render: HashMap<Index, RenderObject>,
}

impl RenderSystem {
    fn new() -> Self {
        Self {
            objects_to_render: HashMap::new(),
        }
    }

    fn create_object_to_render(
        &mut self,
        id: Index,
        mesh: &Mesh,
    ) -> RenderObject {
        println!("Creating new object to render ...");
        let mut gl_object = RenderObject::new();
        unsafe {
            gl::GenVertexArrays(1, &mut gl_object.vao);
            gl::GenBuffers(1, &mut gl_object.vbo);
            gl::GenBuffers(1, &mut gl_object.ebo);
            gl::BindVertexArray(gl_object.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, gl_object.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                   // target
                mesh.size_of_vertices() as gl::types::GLsizeiptr,   // size of data in bytes
                mesh.get_vertices_data().as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                                    // usage
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, gl_object.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,                          // target
                mesh.size_of_indices() as gl::types::GLsizeiptr,   // size of data in bytes
                mesh.indices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                                   // usage
            );

            gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0,         // index of the generic vertex attribute ("layout (location = 0)")
                Mesh::nb_elements_per_vertex(),         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (mesh.size_of_stride()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(), // offset of the first component
            );

            gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
            gl::VertexAttribPointer(
                1,         // index of the generic vertex attribute ("layout (location = 1)")
                Mesh::nb_elements_per_color(),         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (mesh.size_of_stride()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (mesh.color_offset()) as *const gl::types::GLvoid, // offset of the first component
            );

            gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
            gl::VertexAttribPointer(
                2,         // index of the generic vertex attribute ("layout (location =2)")
                Mesh::nb_elements_per_uv(),         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (mesh.size_of_stride()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (mesh.uv_offset()) as *const gl::types::GLvoid, // offset of the first component
            );

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
            gl::BindVertexArray(0);
        }
        self.objects_to_render.insert(id, gl_object);

        return gl_object;
    }


    fn render(&mut self, storage: &mut ComponentStorageManager) {
        for (_, trans) in storage.transform_manager.iter_mut() {
            trans.update_local_transform();
        }

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };

        for (id, mesh) in storage.mesh_manager.iter() {
            let material = storage.get_material(*id).unwrap();
            let transform = storage.get_transform(*id).unwrap();
            let gl_object: RenderObject;
            match self.objects_to_render.get(&id) {
                None => gl_object = self.create_object_to_render(*id, mesh),
                Some(object) => gl_object = *object,
            }
            
            //Compute MVP matrix
            let view_mat = cgmath::Matrix4::from_translation(cgmath::Vector3::new(-1.0, 0.0, -3.0));
            let projection_mat: cgmath::Matrix4<f32> = cgmath::perspective(cgmath::Deg(45.0), 1024.0/768.0, 0.1, 100.0);
            let model_mat = transform.local_transform;

            material.bind();
            material.shader.set_mat4("model", model_mat);
            material.shader.set_mat4("view", view_mat);
            material.shader.set_mat4("projection", projection_mat);

            unsafe {
                gl::BindVertexArray(gl_object.vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    mesh.indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct SceneObject {
    pub id: Index,
}

impl SceneObject {}

pub struct ComponentStorageManager {
    mesh_manager: HashMap<Index, Mesh>,
    shader_manager: HashMap<Index, Shader>,
    transform_manager: HashMap<Index, Transform>,
    material_manager: HashMap<Index, Material>,
    update_manager: HashMap<Index, Box<SceneUpdate>>,
}

impl ComponentStorageManager {
    fn new() -> ComponentStorageManager {
        ComponentStorageManager {
            mesh_manager: HashMap::new(),
            shader_manager: HashMap::new(),
            transform_manager: HashMap::new(),
            material_manager: HashMap::new(),
            update_manager: HashMap::new(),
        }
    }

    fn get_mesh(&self, id: Index) -> Result<&Mesh, String> {
        match self.mesh_manager.get(&id) {
            None => Err("Mesh doesn't exist!".to_string()),
            Some(mesh) => Ok(mesh),
        }
    }

    pub fn get_shader(&self, id: Index) -> Result<&Shader, String> {
        match self.shader_manager.get(&id) {
            None => Err("Shader doesn't exist!".to_string()),
            Some(shader) => Ok(shader),
        }
    }

    pub fn get_transform(&self, id: Index) -> Result<&Transform, String> {
        match self.transform_manager.get(&id) {
            None => Err("Transform doesn't exist!".to_string()),
            Some(transform) => Ok(transform),
        }
    }

    pub fn get_mut_transform(&mut self, id: Index) -> Result<&mut Transform, String> {
        match self.transform_manager.get_mut(&id) {
            None => Err("Transform doesn't exist!".to_string()),
            Some(transform) => Ok(transform),
        }
    }

    fn get_material(&self, id: Index) -> Result<&Material, String> {
        match self.material_manager.get(&id) {
            None => Err("Material doesn't exist!".to_string()),
            Some(material) => Ok(material),
        }
    }

    fn get_update(&self, id: Index) -> Result<&Box<SceneUpdate>, String> {
        match self.update_manager.get(&id) {
            None => Err("Update doesn't exist!".to_string()),
            Some(update) => Ok(update),
        }
    }
}

pub struct Engine {
    window: Window,
    storage: ComponentStorageManager,
    render_system: RenderSystem,
    states_system: StateSystem,
    entity_count: Index,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            window: Window::new(),
            storage: ComponentStorageManager::new(),
            render_system: RenderSystem::new(),
            states_system: StateSystem::new(),
            entity_count: 0,
        }
    }

    pub fn init(&self) {
        self.window.init();
    }

    pub fn start(&mut self) {
        self.main_loop();
    }

    fn main_loop(&mut self) {
        let mut running = true;
        let events = &mut self.window.events_loop;
        let window = &mut self.window.gl_window;
        let start = Instant::now();
        let mut prev_elapsed_time = start.elapsed();
        while running {
            //Calculate delta time since last loop iteration
            let elapsed_time = start.elapsed();
            let delta_time = elapsed_time - prev_elapsed_time;
            if delta_time.subsec_micros() > 0 {
                window.set_title(format!("{} fps ({} ms)", 1000000 / Self::duration_to_micros(delta_time), Self::duration_to_millis(delta_time)).as_str());
            }
            prev_elapsed_time = elapsed_time;

            events.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = window.get_hidpi_factor();
                        window.resize(logical_size.to_physical(dpi_factor));
                    }
                    _ => (),
                },
                _ => (),
            });

            self.states_system.run_update_state(&mut self.storage);
            self.render_system.render(&mut self.storage);

            window.swap_buffers().unwrap();
        }
    }

    fn duration_to_millis(dur: Duration) -> u64 {
        dur.as_secs()*1000 + dur.subsec_millis() as u64
    }

    fn duration_to_micros(dur: Duration) -> u64 {
        dur.as_secs()*1000000 + dur.subsec_micros() as u64
    }

    pub fn create_scene_object(&mut self) -> SceneObject {
        self.entity_count += 1;
        SceneObject {
            id: self.entity_count,
        }
    }

    pub fn create_and_add_mesh(&mut self, id: Index) {
        let mesh = Mesh::new_empty();
        self.storage.mesh_manager.insert(id, mesh);
    }

    pub fn add_mesh(&mut self, id: Index, mesh: Mesh) {
        println!("Mesh manager before update: {} objects", self.storage.mesh_manager.len());
        self.storage.mesh_manager.insert(id, mesh);
        println!("Mesh manager updated: {} objects", self.storage.mesh_manager.len());
    }

    pub fn add_shader(&mut self, id: Index, shader: Shader) {
        self.storage.shader_manager.insert(id, shader);
    }

    pub fn add_transform(&mut self, id: Index, transform: Transform) {
        self.storage.transform_manager.insert(id, transform);
    }

    pub fn add_states<T: EntityState>(&mut self, id: Index, states: T)
    where
        T: 'static,
    {
        self.states_system.set_entity_states(id, states);
    }

    pub fn add_material(&mut self, id: Index, material: Material) {
        self.storage.material_manager.insert(id, material);
    }

    pub fn add_update(&mut self, id: Index, update: Box<SceneUpdate>) {
        self.storage.update_manager.insert(id, update);
    }

    pub fn get_mesh(&self, id: Index) -> &Mesh {
        self.storage.get_mesh(id).unwrap()
    }

    pub fn get_shader(&self, id: Index) -> &Shader {
        self.storage.get_shader(id).unwrap()
    }

    pub fn get_transform(&self, id: Index) -> &Transform {
        self.storage.get_transform(id).unwrap()
    }

    pub fn get_material(&self, id: Index) -> &Material {
        self.storage.get_material(id).unwrap()
    }

    pub fn get_update(&self, id: Index) -> &Box<SceneUpdate> {
        self.storage.get_update(id).unwrap()
    }
}

struct Window {
    width: u32,
    height: u32,
    title: String,
    gl_window: glutin::GlWindow,
    events_loop: glutin::EventsLoop,
}

impl Window {
    fn new() -> Window {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("Matt' Engine")
            .with_dimensions(LogicalSize::new(1024.0, 768.0));
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        Window {
            width: 1024,
            height: 768,
            title: "Matt' Engine".to_string(),
            gl_window: gl_window,
            events_loop: events_loop,
        }
    }

    fn init(&self) {
        unsafe {
            self.gl_window.make_current().unwrap();
        }
        unsafe {
            gl::load_with(|symbol| self.gl_window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Enable(gl::DEPTH_TEST);  
        }
        let gl_version = unsafe {
            CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
                .to_str()
                .unwrap()
        };
        println!("OpenGL version: {}", gl_version);
    }

    fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.gl_window.set_title(title);
    }

}
