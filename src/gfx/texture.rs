extern crate log;
use image::{io::Reader as ImageReader, GenericImageView};
use std::cell::Cell;
use std::cmp;
use std::rc::Rc;

pub(crate) struct NativeTexture {
    handle: u32,
    texture_type: u32,
    width: u32,
    height: u32,
    depth: u32,
    mip_levels: Cell<u32>,
}

impl PartialEq for NativeTexture {
    fn eq(&self, other: &NativeTexture) -> bool {
        self.handle == other.handle
    }
}

#[derive(Clone)]
pub struct Texture {
    pub(crate) handle: Rc<NativeTexture>,
}

impl PartialEq for Texture {
    fn eq(&self, other: &Texture) -> bool {
        self.handle == other.handle
    }
}

impl Texture {
    pub fn from_file(path: &str) -> Option<Texture> {
        let mut handle: u32 = 0;
        let texture_type = gl::TEXTURE_2D;

        unsafe {
            gl::GenTextures(1, &mut handle);
        }

        if handle == 0 {
            None
        } else {
            let image = ImageReader::open(path).unwrap().decode().unwrap();
            let image_dims = image.dimensions();

            unsafe {
                gl::BindTexture(texture_type, handle);

                gl::TexImage2D(
                    texture_type,
                    0,
                    gl::RGBA as i32,
                    image_dims.0 as i32,
                    image_dims.1 as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    image.into_rgba8().as_raw().as_ptr() as *const std::os::raw::c_void,
                );

                gl::TexParameteri(texture_type, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(texture_type, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                gl::TextureParameteri(texture_type, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TextureParameteri(texture_type, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TextureParameteri(texture_type, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);

                gl::BindTexture(texture_type, 0);
            }

            Some(Texture {
                handle: Rc::new(NativeTexture {
                    handle,
                    texture_type,
                    width: image_dims.0,
                    height: image_dims.1,
                    depth: 1,
                    mip_levels: Cell::new(1),
                }),
            })
        }
    }

    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(self.handle.texture_type, self.handle.handle);
        }
    }

    pub fn generate_mipmaps(&mut self) {
        self.bind(0);
        // self.handle.mip_levels = 0;
        self.handle.mip_levels.set(
            ((cmp::max(self.handle.width, self.handle.height) as f32)
                .log2()
                .floor() as u32)
                + 1,
        );

        unsafe {
            gl::GenerateMipmap(self.handle.texture_type);
        }
    }

    pub fn get_handle(&self) -> u32 {
        self.handle.handle
    }

    pub fn get_width(&self) -> u32 {
        self.handle.width
    }

    pub fn get_height(&self) -> u32 {
        self.handle.height
    }

    pub fn get_depth(&self) -> u32 {
        self.handle.depth
    }

    pub fn get_mip_levels(&self) -> u32 {
        self.handle.mip_levels.get()
    }

    pub fn set_min_mag_filters(&self, min_filter: u32, mag_filter: u32) {
        self.bind(0);
        unsafe {
            gl::TexParameteri(
                self.handle.texture_type,
                gl::TEXTURE_MIN_FILTER,
                min_filter as i32,
            );
            gl::TexParameteri(
                self.handle.texture_type,
                gl::TEXTURE_MAG_FILTER,
                mag_filter as i32,
            );
        }
    }

    pub fn set_wrap_modes(&self, s_wrap: u32, t_wrap: u32, r_wrap: u32) {
        self.bind(0);
        unsafe {
            gl::TextureParameteri(self.handle.texture_type, gl::TEXTURE_WRAP_S, s_wrap as i32);
            gl::TextureParameteri(self.handle.texture_type, gl::TEXTURE_WRAP_T, t_wrap as i32);
            gl::TextureParameteri(self.handle.texture_type, gl::TEXTURE_WRAP_R, r_wrap as i32);
        }
    }
}
