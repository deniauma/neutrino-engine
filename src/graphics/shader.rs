extern crate image;

use image::GenericImageView;
use std::ffi::CString;
use math::Mat4;

pub struct MaterialBuilder {}

impl MaterialBuilder {
    pub fn simple_material_2d() -> Material {
        let vert_source = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;
        layout (location = 2) in vec2 aTexCoord;

        out vec3 ourColor;
        out vec2 TexCoord;

        void main()
        {
            gl_Position = vec4(aPos, 1.0);
            ourColor = aColor;
            TexCoord = vec2(aTexCoord.x, aTexCoord.y);
        }
    "#;

        let frag_source = r#"
        #version 330 core
        out vec4 FragColor;

        in vec3 ourColor;
        in vec2 TexCoord;

        // texture sampler
        uniform sampler2D texture1;

        void main()
        {
            FragColor = texture(texture1, TexCoord) * vec4(ourColor, 1.0);
        }
    "#;

        let shader = Shader::new(&vert_source, &frag_source);

        Material::new(shader, Texture::new_empty())
    }

    pub fn simple_texture_material_2d(image_path: &str) -> Material {
        let vert_source = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;
        layout (location = 2) in vec2 aTexCoord;

        out vec3 ourColor;
        out vec2 TexCoord;

        void main()
        {
            gl_Position = vec4(aPos, 1.0);
            ourColor = aColor;
            TexCoord = vec2(aTexCoord.x, aTexCoord.y);
        }
    "#;

        let frag_source = r#"
        #version 330 core
        out vec4 FragColor;

        in vec3 ourColor;
        in vec2 TexCoord;

        // texture sampler
        uniform sampler2D texture1;

        void main()
        {
            FragColor = texture(texture1, TexCoord) * vec4(ourColor, 1.0);
        }
    "#;

        let shader = Shader::new(&vert_source, &frag_source);
        Material::new(shader, Texture::from_file(image_path))
    }
}

#[derive(Copy, Clone)]
pub struct Material {
    shader: Shader,
    texture: Texture,
}

impl Material {
    pub fn new(shader: Shader, texture: Texture) -> Self {
        Self {
            shader: shader,
            texture: texture,
        }
    }

    pub fn bind(&self) {
        self.texture.bind();
        self.shader.use_program();
    }
}

#[derive(Copy, Clone)]
pub struct Texture {
    id: gl::types::GLuint,
}

impl Texture {
    pub fn new_empty() -> Self {
        let white_image: Vec<u8> = vec![255, 255, 255, 255];

        Self {
            id: Texture::generate_gl_texture(white_image, 1, 1),
        }
    }

    pub fn from_file(path: &str) -> Self {
        let img = image::open(path).unwrap();
        let (width, height) = img.dimensions();
        let img = img.to_rgba();

        Self {
            id: Texture::generate_gl_texture(img.into_raw(), width, height),
        }
    }

    fn generate_gl_texture(image: Vec<u8>, width: u32, height: u32) -> gl::types::GLuint {
        let mut texture_id: gl::types::GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to GL_REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const gl::types::GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        return texture_id;
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

#[derive(Copy, Clone)]
pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn new(vertex: &str, fragment: &str) -> Shader {
        let vert_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let frag_id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        let vert_sharder_code = CString::new(vertex).unwrap();
        let frag_sharder_code = CString::new(fragment).unwrap();
        unsafe {
            gl::ShaderSource(vert_id, 1, &vert_sharder_code.as_ptr(), std::ptr::null());
            gl::CompileShader(vert_id);
            gl::ShaderSource(frag_id, 1, &frag_sharder_code.as_ptr(), std::ptr::null());
            gl::CompileShader(frag_id);
        }
        check_compile_status(vert_id).unwrap();
        check_compile_status(frag_id).unwrap();

        let program_id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(program_id, vert_id);
            gl::AttachShader(program_id, frag_id);
            gl::LinkProgram(program_id);
        }
        check_link_status(program_id).unwrap();
        unsafe {
            gl::DetachShader(program_id, vert_id);
            gl::DetachShader(program_id, frag_id);
            gl::DeleteShader(vert_id);
            gl::DeleteShader(frag_id);
        }

        Shader { id: program_id }
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn set_mat4(&self, name: &str, mat: Mat4) {
        let mat_ptr = &mat.data[0][0] as *const f32;
        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()), 1, gl::FALSE, mat_ptr);
        }
    }
}

fn check_compile_status(shader_id: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let buffer: Vec<u8> = vec![b' '; 512];
        let errorlog: CString = unsafe { CString::from_vec_unchecked(buffer) };
        unsafe {
            gl::GetShaderInfoLog(
                shader_id,
                512,
                std::ptr::null_mut(),
                errorlog.as_ptr() as *mut gl::types::GLchar,
            );

            return Err(errorlog.to_string_lossy().into_owned());
        }
    }

    Ok(shader_id)
}

fn check_link_status(program_id: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
    }
    if success == 0 {
        let buffer: Vec<u8> = vec![b' '; 512];
        let errorlog: CString = unsafe { CString::from_vec_unchecked(buffer) };
        unsafe {
            gl::GetProgramInfoLog(
                program_id,
                512,
                std::ptr::null_mut(),
                errorlog.as_ptr() as *mut gl::types::GLchar,
            );

            return Err(errorlog.to_string_lossy().into_owned());
        }
    }

    Ok(program_id)
}
