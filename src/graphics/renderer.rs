use std::collections::HashMap;
use crate::graphics::{Index, ComponentStorageManager, Mesh};



#[derive(Copy, Clone)]
struct RenderObject {
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    ebo: Option<gl::types::GLuint>,
}

impl RenderObject {
    fn new() -> Self {
        Self {
            vao: 0,
            vbo: 0,
            ebo: None,
        }
    }
}

pub struct RenderSystem {
    objects_to_render: HashMap<Index, RenderObject>,
}

impl RenderSystem {
    pub fn new() -> Self {
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
            gl::BindVertexArray(gl_object.vao);
            gl::GenBuffers(1, &mut gl_object.vbo);
            
            gl::BindBuffer(gl::ARRAY_BUFFER, gl_object.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,                                   // target
                mesh.size_of_vertices() as gl::types::GLsizeiptr,   // size of data in bytes
                mesh.get_vertices_data().as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                                    // usage
            );

            //Generate EBO only if indices are present
            if !mesh.indices.is_empty() {
                println!("EBO generated!");
                let mut ebo = 0;
                gl::GenBuffers(1, &mut ebo);
                gl_object.ebo = Some(ebo);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, gl_object.ebo.unwrap());
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,                          // target
                    mesh.size_of_indices() as gl::types::GLsizeiptr,   // size of data in bytes
                    mesh.indices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                    gl::STATIC_DRAW,                                   // usage
                );
            }

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


    pub fn render(&mut self, storage: &mut ComponentStorageManager) {
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
            let view_mat = storage.camera.lookat();
            //let view_mat = cgmath::Matrix4::from_translation(cgmath::Vector3::new(-1.0, 0.0, -3.0));
            let projection_mat: cgmath::Matrix4<f32> = cgmath::perspective(cgmath::Deg(45.0), 1024.0/768.0, 0.1, 100.0);
            let model_mat = transform.local_transform;

            material.bind();
            material.shader.set_mat4("model", model_mat);
            material.shader.set_mat4("view", view_mat);
            material.shader.set_mat4("projection", projection_mat);

            unsafe {
                gl::BindVertexArray(gl_object.vao);
                match gl_object.ebo {
                    None => {gl::DrawArrays(gl::TRIANGLES, 0, mesh.positions.len() as i32)},
                    Some(ebo) => {gl::DrawElements(gl::TRIANGLES, mesh.indices.len() as i32, gl::UNSIGNED_INT,std::ptr::null())},
                }
            }
        }
        unsafe {
            match gl::GetError(){
                gl::NO_ERROR => (),
                gl::INVALID_ENUM => println!("OpenGL error: INVALID_ENUM"),
                gl::INVALID_VALUE => println!("OpenGL error: INVALID_VALUE"),
                gl::INVALID_OPERATION => println!("OpenGL error: INVALID_OPERATION"),
                gl::INVALID_FRAMEBUFFER_OPERATION => println!("OpenGL error: INVALID_FRAMEBUFFER_OPERATION"),
                gl::OUT_OF_MEMORY => println!("OpenGL error: OUT_OF_MEMORY"),
                _ => println!("OpenGL error: other"),
            }
        }
    }
}