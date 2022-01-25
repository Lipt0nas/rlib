extern crate log;
use image::{io::Reader as ImageReader, GenericImageView};
use std::cmp;

pub struct Texture {
    handle: u32,
    texture_type: u32,
    width: u32,
    height: u32,
    depth: u32,
    mip_levels: u32,
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
                handle,
                texture_type,
                width: image_dims.0,
                height: image_dims.1,
                depth: 1,
                mip_levels: 1,
            })
        }
    }

    pub fn bind(&self, slot: u32) {
        unsafe {
            gl::ActiveTexture(slot);
            gl::BindTexture(self.texture_type, self.handle);
        }
    }

    pub fn generate_mipmaps(&mut self) {
        self.bind(0);

        self.mip_levels = ((cmp::max(self.width, self.height) as f32).log2().floor() as u32) + 1;

        unsafe {
            gl::GenerateMipmap(self.texture_type);
        }
    }

    pub fn get_handle(&self) -> u32 {
        self.handle
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    pub fn get_mip_levels(&self) -> u32 {
        self.mip_levels
    }

    pub fn set_min_mag_filters(&self, min_filter: u32, mag_filter: u32) {
        self.bind(0);
        unsafe {
            gl::TexParameteri(self.texture_type, gl::TEXTURE_MIN_FILTER, min_filter as i32);
            gl::TexParameteri(self.texture_type, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
        }
    }

    pub fn set_wrap_modes(&self, s_wrap: u32, t_wrap: u32, r_wrap: u32) {
        self.bind(0);
        unsafe {
            gl::TextureParameteri(self.texture_type, gl::TEXTURE_WRAP_S, s_wrap as i32);
            gl::TextureParameteri(self.texture_type, gl::TEXTURE_WRAP_T, t_wrap as i32);
            gl::TextureParameteri(self.texture_type, gl::TEXTURE_WRAP_R, r_wrap as i32);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        info!("Dropping texture");
        if self.handle != 0 {
            unsafe {
                gl::DeleteTextures(1, &self.handle);
            }
        }
    }
}
