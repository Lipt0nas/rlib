extern crate log;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    pub fn from_float_bits(bits: f32) -> Color {
        let bits: u32 = bits.to_bits();

        let r = ((bits & 0xFF000000) >> 24) as f32;
        let g = ((bits & 0x00FF0000) >> 16) as f32;
        let b = ((bits & 0x0000FF00) >> 8) as f32;
        let a = (bits & 0x000000FF) as f32;

        Color { r, g, b, a }
    }

    pub fn to_rgba8(self) -> f32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        let a = (self.a * 255.0) as u32;

        let mut bits: u32 = 0;
        bits |= r << 24;
        bits |= g << 16;
        bits |= b << 8;
        bits |= a;

        f32::from_bits(bits)
    }
}
