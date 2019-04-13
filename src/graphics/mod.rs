extern crate gl;
extern crate glutin;

use glutin::dpi::*;
use glutin::GlContext;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::time::{Duration, Instant};

pub mod mesh;
pub mod shader;
use crate::graphics::shader::*;

type Index = u32;


pub struct SimpleState {
    update: fn(),
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
        mesh: &mesh::Mesh,
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
                mesh::Mesh::nb_elements_per_vertex(),         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (mesh.size_of_stride()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(), // offset of the first component
            );

            gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
            gl::VertexAttribPointer(
                1,         // index of the generic vertex attribute ("layout (location = 1)")
                mesh::Mesh::nb_elements_per_color(),         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (mesh.size_of_stride()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                (mesh.color_offset()) as *const gl::types::GLvoid, // offset of the first component
            );

            gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
            gl::VertexAttribPointer(
                2,         // index of the generic vertex attribute ("layout (location =2)")
                mesh::Mesh::nb_elements_per_uv(),         // the number of components per generic vertex attribute
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

    fn render(&mut self, storage: &ComponentStorageManager) {
        for (id, mesh) in storage.mesh_manager.iter() {
            let material = storage.get_material(*id).unwrap();
            let gl_object: RenderObject;
            match self.objects_to_render.get(&id) {
                None => gl_object = self.create_object_to_render(*id, mesh),
                Some(object) => gl_object = *object,
            }
            unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
            material.bind();
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

struct ComponentStorageManager {
    mesh_manager: HashMap<Index, mesh::Mesh>,
    shader_manager: HashMap<Index, shader::Shader>,
    state_manager: HashMap<Index, SimpleState>,
    material_manager: HashMap<Index, Material>,
}

impl ComponentStorageManager {
    fn new() -> ComponentStorageManager {
        ComponentStorageManager {
            mesh_manager: HashMap::new(),
            shader_manager: HashMap::new(),
            state_manager: HashMap::new(),
            material_manager: HashMap::new(),
        }
    }

    fn get_mesh(&self, id: Index) -> Result<&mesh::Mesh, String> {
        match self.mesh_manager.get(&id) {
            None => Err("Mesh doesn't exist!".to_string()),
            Some(mesh) => Ok(mesh),
        }
    }

    pub fn get_shader(&self, id: Index) -> Result<&shader::Shader, String> {
        match self.shader_manager.get(&id) {
            None => Err("Shader doesn't exist!".to_string()),
            Some(shader) => Ok(shader),
        }
    }

    fn get_state(&self, id: Index) -> Result<&SimpleState, String> {
        match self.state_manager.get(&id) {
            None => Err("State doesn't exist!".to_string()),
            Some(state) => Ok(state),
        }
    }

    fn get_material(&self, id: Index) -> Result<&Material, String> {
        match self.material_manager.get(&id) {
            None => Err("Material doesn't exist!".to_string()),
            Some(material) => Ok(material),
        }
    }
}

pub struct Engine {
    window: Window,
    storage: ComponentStorageManager,
    render_system: RenderSystem,
    entity_count: Index,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            window: Window::new(),
            storage: ComponentStorageManager::new(),
            render_system: RenderSystem::new(),
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

            self.render_system.render(&self.storage);

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
        let mesh = mesh::Mesh::new_empty();
        self.storage.mesh_manager.insert(id, mesh);
    }

    pub fn add_mesh(&mut self, id: Index, mesh: mesh::Mesh) {
        println!("Mesh manager before update: {} objects", self.storage.mesh_manager.len());
        self.storage.mesh_manager.insert(id, mesh);
        println!("Mesh manager updated: {} objects", self.storage.mesh_manager.len());
    }

    pub fn add_shader(&mut self, id: Index, shader: shader::Shader) {
        self.storage.shader_manager.insert(id, shader);
    }

    pub fn add_state(&mut self, id: Index, state: SimpleState) {
        self.storage.state_manager.insert(id, state);
    }

    pub fn add_material(&mut self, id: Index, material: Material) {
        self.storage.material_manager.insert(id, material);
    }

    pub fn get_mesh(&self, id: Index) -> &mesh::Mesh {
        self.storage.get_mesh(id).unwrap()
    }

    pub fn get_shader(&self, id: Index) -> &shader::Shader {
        self.storage.get_shader(id).unwrap()
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
            println!("Test before laoding fl fn");
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
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
