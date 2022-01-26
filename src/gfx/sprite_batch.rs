use super::buffer::Buffer;
use super::color::Color;
extern crate log;

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
}

impl<const COUNT: usize> SpriteBatch<COUNT> {
    pub fn new() -> Option<SpriteBatch<COUNT>> {
        Some(SpriteBatch {
            vertex_buffer: Buffer::new_with_capacity(
                gl::ARRAY_BUFFER,
                gl::DYNAMIC_DRAW,
                (COUNT * std::mem::size_of::<f32>()) as u32,
            )
            .unwrap(),
            index_buffer: Buffer::new_with_capacity(
                gl::ELEMENT_ARRAY_BUFFER,
                gl::STATIC_DRAW,
                (COUNT * std::mem::size_of::<u32>() * 6) as u32,
            )
            .unwrap(),
            vertices: [SpriteVertex {
                x: 0.0,
                y: 0.0,
                u: 0.0,
                v: 0.0,
                color: 0.0,
            }; COUNT],
            vao_handle: 0,
            vertex_offset: 0,
        })
    }

    pub fn initialize(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao_handle);
            gl::BindVertexArray(self.vao_handle);

            self.vertex_buffer.bind();

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
            self.vertex_buffer.unbind();
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

        self.index_buffer.set_data_u32(indices.as_slice());
    }

    pub fn begin_batch(&mut self) {
        self.vertex_offset = 0;
    }

    pub fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let color = Color::from_rgba(0.0, 0.0, 0.0, 1.0);
        let c = color.to_rgba8() as f32;

        info!("{}", c);

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

    pub fn end_batch(&mut self) {
        unsafe {
            let ptr = std::slice::from_raw_parts(
                (&self.vertices[0] as *const SpriteVertex) as *const f32,
                (self.vertex_offset * 5) as usize,
            );

            //info!("{:?}", ptr);

            self.vertex_buffer.bind();
            self.vertex_buffer.copy_data_part(
                ptr,
                (self.vertex_offset * (std::mem::size_of::<f32>() * 5) as u32) as isize,
                0,
            );

            let offset = 0;

            gl::BindVertexArray(self.vao_handle);
            self.index_buffer.bind();
            gl::DrawElements(
                gl::TRIANGLES,
                ((self.vertex_offset / 4) * 6) as i32,
                gl::UNSIGNED_INT,
                offset as *const std::os::raw::c_void,
            );
        }
    }
}
