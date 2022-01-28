use super::texture::Texture;

extern crate log;

#[derive(Clone)]
pub struct TextureRegion {
    texture: Texture,
    u: f32,
    v: f32,
    u2: f32,
    v2: f32,
}

impl TextureRegion {
    pub fn new(texture: Texture) -> TextureRegion {
        TextureRegion {
            texture,
            u: 0.0,
            v: 0.0,
            u2: 1.0,
            v2: 1.0,
        }
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn get_u(&self) -> f32 {
        self.u
    }

    pub fn get_v(&self) -> f32 {
        self.v
    }

    pub fn get_u2(&self) -> f32 {
        self.u2
    }

    pub fn get_v2(&self) -> f32 {
        self.v2
    }
}
