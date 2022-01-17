use rlib::*;

struct Application {
    vertex_buffer: gfx::buffer::Buffer,
    shader_program: gfx::shader::ShaderProgram,
    vao: u32,
}

impl rlib::RLibApp for Application {
    fn new() -> Self {
        let vertex_shader = gfx::shader::Shader::from_string(
            gl::VERTEX_SHADER,
            "
            #version 430 core
            
            layout (location = 0) in vec3 in_position;

            layout (location = 0) out vec3 out_color;

            const vec3[3] color_map = vec3[] (
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 0.0, 1.0)
            );

            void main() {
                gl_Position = vec4(in_position, 1.0);

                out_color = color_map[gl_VertexID];
            }
        ",
        )
        .unwrap();

        let fragment_shader = gfx::shader::Shader::from_string(
            gl::FRAGMENT_SHADER,
            "
            #version 430 core

            layout (location = 0) in vec3 in_color;

            layout (location = 0) out vec4 out_color;

            void main() {
                out_color = vec4(in_color, 1.0);
            }
        ",
        )
        .unwrap();

        Self {
            vertex_buffer: gfx::buffer::Buffer::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW).unwrap(),
            shader_program: gfx::shader::ShaderProgram::from_shaders(&[
                vertex_shader,
                fragment_shader,
            ])
            .unwrap(),
            vao: 0,
        }
    }

    fn init(&mut self) {
        let data: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        self.vertex_buffer.set_data(data.as_slice());
        self.vertex_buffer.unbind();

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }

        self.vertex_buffer.bind();
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null(),
            );
            self.vertex_buffer.unbind();
            gl::BindVertexArray(0);
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.shader_program.bind();
            gl::BindVertexArray(self.vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn main() {
    let config = rlib::RlibConfig {
        window_title: "Rlib Test".to_string(),
        window_width: 1600,
        window_height: 900,
    };

    rlib::init::<Application>(config);
}
