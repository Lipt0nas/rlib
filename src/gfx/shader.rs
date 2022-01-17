extern crate log;

use std::ffi::CString;

pub struct Shader {
    handle: u32,
    shader_type: u32,
    source: String,
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
                handle: handle,
                shader_type: shader_type,
                source: source.to_owned(),
            })
        }
    }

    pub fn get_handle(&self) -> u32 {
        self.handle
    }

    pub fn get_type(&self) -> u32 {
        self.shader_type
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        info!("Dropping shader");
        if self.handle != 0 {
            unsafe {
                gl::DeleteShader(self.handle);
            }
        }
    }
}

pub struct ShaderProgram {
    handle: u32,
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

        Some(ShaderProgram { handle: handle })
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        info!("Dropping shader program");
        if self.handle != 0 {
            unsafe {
                gl::DeleteProgram(self.handle);
            }
        }
    }
}
