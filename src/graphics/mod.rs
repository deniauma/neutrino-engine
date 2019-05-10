extern crate gl;
extern crate glutin;

use glutin::dpi::*;
use glutin::GlContext;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::time::{Duration, Instant};
use crate::server::debug;

pub mod mesh;
pub mod shader;
pub mod transform;
pub mod scene;
pub mod states;
pub mod primitives;
pub mod entity;
pub mod camera;
pub mod inputs;
pub mod renderer;

use self::transform::Transform;
use self::mesh::*;
use self::shader::*;
use self::scene::*;
use self::states::*;
use self::primitives::*;
use self::camera::Camera;
use self::inputs::InputSystem;
use self::renderer::RenderSystem;


pub type Index = u32;

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
    camera: Camera,
}

impl ComponentStorageManager {
    fn new() -> ComponentStorageManager {
        ComponentStorageManager {
            mesh_manager: HashMap::new(),
            shader_manager: HashMap::new(),
            transform_manager: HashMap::new(),
            material_manager: HashMap::new(),
            update_manager: HashMap::new(),
            camera: Camera::default(),
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

    pub fn get_material(&self, id: Index) -> Result<&Material, String> {
        match self.material_manager.get(&id) {
            None => Err("Material doesn't exist!".to_string()),
            Some(material) => Ok(material),
        }
    }

    pub fn get_update(&self, id: Index) -> Result<&Box<SceneUpdate>, String> {
        match self.update_manager.get(&id) {
            None => Err("Update doesn't exist!".to_string()),
            Some(update) => Ok(update),
        }
    }
    
    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_mut_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }
}

pub type GameData<'a> = (Index, &'a mut ComponentStorageManager, &'a InputSystem);

pub struct Engine {
    window: Window,
    storage: ComponentStorageManager,
    render_system: RenderSystem,
    states_system: StateSystem,
    input_system: InputSystem,
    entity_count: Index,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            window: Window::new(),
            storage: ComponentStorageManager::new(),
            render_system: RenderSystem::new(),
            states_system: StateSystem::new(),
            input_system: InputSystem::new() ,
            entity_count: 0,
        }
    }

    pub fn init(&self) {
        self.window.init();
    }

    pub fn start(&mut self) {
        self.main_loop();
    }

    pub fn start_debug_server(&self) {
        std::thread::spawn(|| {
            debug::open();
        });
    }

    fn main_loop(&mut self) {
        let mut running = true;

        let start = Instant::now();
        let mut prev_elapsed_time = start.elapsed();
        while running {
            //Calculate delta time since last loop iteration
            let elapsed_time = start.elapsed();
            let delta_time = elapsed_time - prev_elapsed_time;
            prev_elapsed_time = elapsed_time;
            running = self.manage_events();
            
            let delta = (delta_time.as_millis() as f32) / 1000.0;
            self.states_system.run_update_state(&mut self.storage, &self.input_system, delta);
            self.render_system.render(&mut self.storage);

            self.window.gl_window.swap_buffers().unwrap();
            if delta_time.subsec_micros() > 0 {
                self.window.gl_window.set_title(format!("{} fps ({} ms)", 1000000 / Self::duration_to_micros(delta_time), Self::duration_to_millis(delta_time)).as_str());
            }
        }
    }

    fn manage_events(&mut self) -> bool {
        let events = &mut self.window.events_loop;
        let window = &mut self.window.gl_window;
        let inputs = &mut self.input_system;
        let mut running = true;

        events.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::Resized(logical_size) => {
                    let dpi_factor = window.get_hidpi_factor();
                    window.resize(logical_size.to_physical(dpi_factor));
                }
                glutin::WindowEvent::KeyboardInput {input, ..} => inputs.set_key_event(input),
                glutin::WindowEvent::MouseInput { button, state, .. } => inputs.set_mouse_button_event(button, state),
                glutin::WindowEvent::MouseWheel { delta, ..} => (),
                _ => (),
            },
            glutin::Event::DeviceEvent { event, .. } => match event {
                glutin::DeviceEvent::MouseMotion { delta } => inputs.set_mouse_move_event(delta.0, delta.1),
                _ => (),
            }
            _ => (),
        });
        running
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
