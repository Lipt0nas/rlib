use super::buffer::Buffer;
use super::color::colors;
use super::color::Color;
use super::shader::Shader;
use super::shader::ShaderProgram;
use super::texture::Texture;
use super::texture_region::TextureRegion;
use glam::{Vec2, Vec4};

#[derive(Clone)]
pub struct Sprite {
    pub region: TextureRegion,
    pub position: Vec2,
    pub size: Vec2,
    pub origin: Vec2,
    pub rotation: f32,
    pub color: Color,
}

impl Sprite {
    pub fn new(texture: Texture) -> Sprite {
        let width = texture.get_width();
        let height = texture.get_height();
        Sprite {
            region: TextureRegion::new(texture),
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(width as f32, height as f32),
            color: Color::from_rgba(1.0, 1.0, 1.0, 1.0),
            origin: Vec2::new(width as f32 / 2.0, height as f32 / 2.0),
            rotation: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct SpriteVertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
    pub color: f32,
}

pub struct SpriteBatch<const COUNT: usize> {
    vertices: [SpriteVertex; COUNT],
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    vao_handle: u32,
    vertex_offset: u32,
    last_texture: Option<Texture>,
    drawing: bool,
    shader_program: ShaderProgram,
}

impl<const COUNT: usize> SpriteBatch<COUNT> {
    pub fn new() -> Option<SpriteBatch<COUNT>> {
        let vertex_shader = Shader::from_string(
            gl::VERTEX_SHADER,
            "
            #version 430 core
            
            layout (location = 0) in vec2 in_position;
            layout (location = 1) in vec2 in_uv;
            layout (location = 2) in vec4 in_color;

            layout (location = 0) out vec4 out_color;
            layout (location = 1) out vec2 out_uv;

            void main() {
                gl_Position = vec4(in_position, 0.0, 1.0);

                out_color = in_color;
                out_uv = in_uv;
            }
        ",
        )
        .unwrap();

        let fragment_shader = Shader::from_string(
            gl::FRAGMENT_SHADER,
            "
            #version 430 core

            layout (location = 0) in vec4 in_color;
            layout (location = 1) in vec2 in_uv;

            layout (location = 0) out vec4 out_color;

            uniform sampler2D u_texture;

            void main() {
                vec4 color = texture(u_texture, in_uv) * in_color;
                out_color = color;
            }
        ",
        )
        .unwrap();

        let vertex_buffer = Buffer::new_with_capacity(
            gl::ARRAY_BUFFER,
            gl::DYNAMIC_DRAW,
            (COUNT * std::mem::size_of::<SpriteVertex>()) as u32,
        )
        .unwrap();
        let mut index_buffer = Buffer::new_with_capacity(
            gl::ELEMENT_ARRAY_BUFFER,
            gl::STATIC_DRAW,
            (COUNT * std::mem::size_of::<u32>() * 6) as u32,
        )
        .unwrap();

        let mut vao_handle = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao_handle);
            gl::BindVertexArray(vao_handle);

            vertex_buffer.bind();

            let pos_offset = 0;
            let uv_offset = 2 * std::mem::size_of::<f32>();
            let color_offset = 4 * std::mem::size_of::<f32>();

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
                pos_offset as *const std::os::raw::c_void,
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
                uv_offset as *const std::os::raw::c_void,
            );

            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                4,
                gl::UNSIGNED_BYTE,
                gl::TRUE,
                (5 * std::mem::size_of::<f32>()) as gl::types::GLint,
                color_offset as *const std::os::raw::c_void,
            );
            gl::BindVertexArray(0);
            vertex_buffer.unbind();
        }

        let mut indices: Vec<u32> = vec![0; COUNT * 6];

        let mut i: u32 = 0;
        let mut j: u32 = 0;

        loop {
            indices[(i) as usize] = j;
            indices[(i + 1) as usize] = j + 1;
            indices[(i + 2) as usize] = j + 2;
            indices[(i + 3) as usize] = j + 2;
            indices[(i + 4) as usize] = j + 3;
            indices[(i + 5) as usize] = j;

            i += 6;
            j += 4;

            if i == (COUNT * 6) as u32 {
                break;
            }
        }

        index_buffer.set_data_u32(indices.as_slice());

        Some(SpriteBatch {
            vertex_buffer,
            index_buffer,
            vao_handle,
            vertices: [SpriteVertex {
                x: 0.0,
                y: 0.0,
                u: 0.0,
                v: 0.0,
                color: 0.0,
            }; COUNT],
            vertex_offset: 0,
            last_texture: None,
            drawing: false,
            shader_program: ShaderProgram::from_shaders(&[fragment_shader, vertex_shader]).unwrap(),
        })
    }

    pub fn begin_batch(&mut self) {
        if !self.drawing {
            self.drawing = true;
        } else {
            panic!("Can't call begin_batch() on a batch that is already drawing!");
        }
    }

    pub fn draw(
        &mut self,
        texture: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Option<Color>,
    ) {
        self.check_state(texture);

        let color = match color {
            Some(c) => c,
            None => colors::WHITE,
        };

        let c = color.to_rgba8();

        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = x;
            vtx.y = y;
            vtx.u = 0.0;
            vtx.v = 1.0;
            vtx.color = c;

            self.vertex_offset += 1;
        }
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = x + width;
            vtx.y = y;
            vtx.u = 1.0;
            vtx.v = 1.0;
            vtx.color = c;

            self.vertex_offset += 1;
        }
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = x + width;
            vtx.y = y + height;
            vtx.u = 1.0;
            vtx.v = 0.0;
            vtx.color = c;

            self.vertex_offset += 1;
        }
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = x;
            vtx.y = y + height;
            vtx.u = 0.0;
            vtx.v = 0.0;
            vtx.color = c;

            self.vertex_offset += 1;
        }
    }

    pub fn draw_region(
        &mut self,
        region: &TextureRegion,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Option<Color>,
    ) {
        self.draw(region.get_texture(), x, y, width, height, color);
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        self.check_state(sprite.region.get_texture());

        let c = sprite.color.to_rgba8();

        let origin = Vec2::new(
            sprite.origin.x + sprite.position.x,
            sprite.origin.y + sprite.position.y,
        );

        let offsets = Vec4::new(
            -origin.x,
            -origin.y,
            sprite.size.x - origin.x,
            sprite.size.y - origin.y,
        );

        let rot = (-sprite.rotation).to_radians();
        let r_sin = rot.sin();
        let r_cos = rot.cos();

        let p1 = Vec2::new(
            (offsets.x * r_cos - offsets.y * r_sin) + origin.x,
            (offsets.x * r_sin + offsets.y * r_cos) + origin.y,
        );

        let p2 = Vec2::new(
            (offsets.z * r_cos - offsets.y * r_sin) + origin.x,
            (offsets.z * r_sin + offsets.y * r_cos) + origin.y,
        );

        let p3 = Vec2::new(
            (offsets.z * r_cos - offsets.w * r_sin) + origin.x,
            (offsets.z * r_sin + offsets.w * r_cos) + origin.y,
        );

        let p4 = Vec2::new(
            (offsets.x * r_cos - offsets.w * r_sin) + origin.x,
            (offsets.x * r_sin + offsets.w * r_cos) + origin.y,
        );

        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = p1.x;
            vtx.y = p1.y;
            vtx.u = sprite.region.get_u();
            vtx.v = sprite.region.get_v();
            vtx.color = c;

            self.vertex_offset += 1;
        }
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = p2.x;
            vtx.y = p2.y;
            vtx.u = sprite.region.get_u2();
            vtx.v = sprite.region.get_v();
            vtx.color = c;

            self.vertex_offset += 1;
        }
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = p3.x;
            vtx.y = p3.y;
            vtx.u = sprite.region.get_u2();
            vtx.v = sprite.region.get_v2();
            vtx.color = c;

            self.vertex_offset += 1;
        }
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = p4.x;
            vtx.y = p4.y;
            vtx.u = sprite.region.get_u();
            vtx.v = sprite.region.get_v2();
            vtx.color = c;

            self.vertex_offset += 1;
        }
    }

    pub fn end_batch(&mut self) {
        if !self.drawing {
            panic!("Can't call end_batch() on a batch that is not in the drawing state!");
        }

        self.flush_batch();

        self.drawing = false;
    }

    pub fn flush_batch(&mut self) {
        if self.vertex_offset > 0 {
            unsafe {
                let ptr = std::slice::from_raw_parts(
                    (&self.vertices[0] as *const SpriteVertex) as *const f32,
                    (self.vertex_offset as usize * std::mem::size_of::<SpriteVertex>()) as usize,
                );

                self.vertex_buffer.bind();
                self.vertex_buffer.copy_data_part(
                    ptr,
                    (self.vertex_offset * (std::mem::size_of::<SpriteVertex>()) as u32) as isize,
                    0,
                );

                let offset = 0;

                gl::BindVertexArray(self.vao_handle);
                self.index_buffer.bind();

                self.shader_program.bind();
                self.shader_program.set_float("u_t", 100.0);

                self.last_texture.as_ref().unwrap().bind(0);

                gl::DrawElements(
                    gl::TRIANGLES,
                    ((self.vertex_offset / 4) * 6) as i32,
                    gl::UNSIGNED_INT,
                    offset as *const std::os::raw::c_void,
                );
            }
        }

        self.vertex_offset = 0;
        self.last_texture = None;
    }

    fn check_state(&mut self, texture: &Texture) {
        if !self.drawing {
            panic!("Can't issue draw commands to a batch that is not in the drawing state!");
        }

        if let Some(tex) = &self.last_texture {
            if texture != tex {
                self.flush_batch();
            }
        }

        self.last_texture = Some(texture.clone());
    }
}
