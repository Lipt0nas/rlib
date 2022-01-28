extern crate log;

use std::{collections::HashMap, ffi::CString, rc::Rc};

pub(crate) struct NativeShader {
    handle: u32,
    shader_type: u32,
    source: String,
}

#[derive(Clone)]
pub struct Shader {
    handle: Rc<NativeShader>,
}

impl Shader {
    #[allow(temporary_cstring_as_ptr)]
    pub fn from_string(shader_type: u32, source: &str) -> Option<Shader> {
        let handle: u32 = unsafe { gl::CreateShader(shader_type) };

        if handle == 0 {
            None
        } else {
            let mut status: i32 = 1;
            unsafe {
                gl::ShaderSource(
                    handle,
                    1,
                    &CString::new(source).unwrap().as_ptr(),
                    std::ptr::null(),
                );
                gl::CompileShader(handle);
                gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut status);
            }

            if status == 0 {
                let mut error_len: i32 = 0;

                unsafe {
                    gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut error_len);
                }

                let mut error_buffer: Vec<u8> = Vec::with_capacity(error_len as usize + 1);
                error_buffer.extend([b' '].iter().cycle().take(error_len as usize));

                let error: CString = unsafe { CString::from_vec_unchecked(error_buffer) };

                unsafe {
                    gl::GetShaderInfoLog(
                        handle,
                        error_len,
                        std::ptr::null_mut(),
                        error.as_ptr() as *mut gl::types::GLchar,
                    );
                }

                error!("Error compiling shader: {}", error.to_string_lossy());

                unsafe {
                    gl::DeleteShader(handle);
                }

                return None;
            }

            Some(Shader {
                handle: Rc::new(NativeShader {
                    handle,
                    shader_type,
                    source: source.to_owned(),
                }),
            })
        }
    }

    pub fn get_handle(&self) -> u32 {
        self.handle.handle
    }

    pub fn get_type(&self) -> u32 {
        self.handle.shader_type
    }

    pub fn get_source(&self) -> &str {
        &self.handle.source
    }
}

pub(crate) struct NativeShaderProgram {
    handle: u32,
    uniforms: HashMap<String, u32>,
}

#[derive(Clone)]
pub struct ShaderProgram {
    handle: Rc<NativeShaderProgram>,
}

impl ShaderProgram {
    pub fn from_shaders(shaders: &[Shader]) -> Option<ShaderProgram> {
        let handle = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(handle, shader.get_handle());
            }
        }

        unsafe {
            gl::LinkProgram(handle);
        }

        let mut status: i32 = 1;
        unsafe {
            gl::GetProgramiv(handle, gl::LINK_STATUS, &mut status);
        }

        if status == 0 {
            let mut error_len: i32 = 0;
            unsafe {
                gl::GetProgramiv(handle, gl::INFO_LOG_LENGTH, &mut error_len);
            }

            let mut error_buffer: Vec<u8> = Vec::with_capacity(error_len as usize + 1);
            error_buffer.extend([b' '].iter().cycle().take(error_len as usize));

            let error: CString = unsafe { CString::from_vec_unchecked(error_buffer) };

            unsafe {
                gl::GetProgramInfoLog(
                    handle,
                    error_len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            error!("Error linking shader program: {}", error.to_string_lossy());

            unsafe {
                gl::DeleteProgram(handle);
            }

            return None;
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(handle, shader.get_handle());
            }
        }

        let mut uniforms: HashMap<String, u32> = HashMap::new();
        let mut uniform_count: i32 = 0;
        unsafe {
            gl::GetProgramiv(handle, gl::ACTIVE_UNIFORMS, &mut uniform_count);

            let mut string_buffer: Vec<u8> = Vec::with_capacity(256);
            string_buffer.extend([b' '].iter().cycle().take(255));

            let var_name: CString = CString::from_vec_unchecked(string_buffer);

            for i in 0..uniform_count {
                let mut name_len: i32 = 0;
                let mut var_size: i32 = 0;
                let mut var_type: u32 = 0;

                gl::GetActiveUniform(
                    handle,
                    i as u32,
                    256,
                    &mut name_len,
                    &mut var_size,
                    &mut var_type,
                    var_name.as_ptr() as *mut gl::types::GLchar,
                );

                let var_loc = gl::GetUniformLocation(handle, var_name.as_ptr());

                {
                    let mut string = var_name.to_string_lossy().into_owned();
                    string.truncate(name_len as usize);

                    uniforms.insert(string, var_loc as u32);
                }
            }
        }

        Some(ShaderProgram {
            handle: Rc::new(NativeShaderProgram { handle, uniforms }),
        })
    }

    pub fn set_float(&self, name: &str, value: f32) {
        if let Some(uniform) = self.handle.uniforms.get(name) {
            unsafe {
                gl::Uniform1f(*uniform as i32, value);
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle.handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}
