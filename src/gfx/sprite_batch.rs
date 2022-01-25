use std::mem::size_of;

use super::buffer::Buffer;
extern crate log;

#[derive(Copy, Clone)]
#[repr(C)]
struct SpriteVertex {
    pub x: f32,
    pub y: f32,
    pub u: f32,
    pub v: f32,
}

pub struct SpriteBatch<const COUNT: usize> {
    vertices: [SpriteVertex; COUNT],
    vertex_buffer: Buffer,
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
            vertices: [SpriteVertex {
                x: 0.0,
                y: 0.0,
                u: 0.0,
                v: 0.0,
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

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                pos_offset as *const std::os::raw::c_void,
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as gl::types::GLint,
                uv_offset as *const std::os::raw::c_void,
            );
            gl::BindVertexArray(0);
            self.vertex_buffer.unbind();
        }
    }

    pub fn begin_batch(&mut self) {
        self.vertex_offset = 0;
    }

    pub fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = x;
            vtx.y = y;
            vtx.u = 0.0;
            vtx.v = 0.0;

            self.vertex_offset += 1;
        }
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = x + width;
            vtx.y = y;
            vtx.u = 0.0;
            vtx.v = 0.0;

            self.vertex_offset += 1;
        }
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = x + width;
            vtx.y = y + height;
            vtx.u = 0.0;
            vtx.v = 0.0;

            self.vertex_offset += 1;
        }
        {
            let vtx: &mut SpriteVertex = &mut self.vertices[self.vertex_offset as usize];

            vtx.x = x;
            vtx.y = y + height;
            vtx.u = 0.0;
            vtx.v = 0.0;

            self.vertex_offset += 1;
        }
    }

    pub fn end_batch(&mut self) {
        unsafe {
            let ptr = std::slice::from_raw_parts(
                (&self.vertices[0] as *const SpriteVertex) as *const f32,
                (self.vertex_offset * 6) as usize,
            );

            //info!("{:?}", ptr);

            self.vertex_buffer.bind();
            self.vertex_buffer.copy_data_part(
                ptr,
                (self.vertex_offset * (std::mem::size_of::<f32>() * 4) as u32) as isize,
                0,
            );
            gl::BindVertexArray(self.vao_handle);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, self.vertex_offset as i32);
        }
    }
}
