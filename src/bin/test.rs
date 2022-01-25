use rlib::*;

struct Application {
    vertex_buffer: gfx::buffer::Buffer,
    shader_program: gfx::shader::ShaderProgram,
    texture: gfx::texture::Texture,
    batch: gfx::sprite_batch::SpriteBatch<1000>,
    vao: u32,
    test_val: f32,
}

impl rlib::RLibApp for Application {
    fn new() -> Self {
        let vertex_shader = gfx::shader::Shader::from_string(
            gl::VERTEX_SHADER,
            "
            #version 430 core
            
            layout (location = 0) in vec2 in_position;
            layout (location = 1) in vec2 in_uv;

            layout (location = 0) out vec3 out_color;
            layout (location = 1) out vec2 out_uv;

            const vec3[3] color_map = vec3[] (
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 0.0, 1.0)
            );

            void main() {
                gl_Position = vec4(in_position, 0.0, 1.0);

                out_color = color_map[gl_VertexID];
                out_uv = in_uv;
            }
        ",
        )
        .unwrap();

        let fragment_shader = gfx::shader::Shader::from_string(
            gl::FRAGMENT_SHADER,
            "
            #version 430 core

            layout (location = 0) in vec3 in_color;
            layout (location = 1) in vec2 in_uv;

            layout (location = 0) out vec4 out_color;

            uniform sampler2D u_texture;

            void main() {
                out_color = texture(u_texture, in_uv) * vec4(in_color, 1.0);
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
            texture: gfx::texture::Texture::from_file("data/amogus.png").unwrap(),
            vao: 0,
            batch: gfx::sprite_batch::SpriteBatch::new().unwrap(),
            test_val: 0.0,
        }
    }

    fn init(&mut self) {
        let data: Vec<f32> = vec![
            -0.5, -0.5, 0.0, 0.0, 0.0, 0.5, -0.5, 0.0, 1.0, 1.0, 0.0, 0.5, 0.0, 1.0, 0.0,
        ];

        self.vertex_buffer.set_data(data.as_slice());
        self.vertex_buffer.unbind();

        self.texture.set_min_mag_filters(gl::NEAREST, gl::NEAREST);

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
        }

        self.vertex_buffer.bind();
        unsafe {
            let pos_offset = 0;
            let uv_offset = 3 * std::mem::size_of::<f32>();

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
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
            self.vertex_buffer.unbind();
            gl::BindVertexArray(0);
        }

        self.batch.initialize();
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            self.shader_program.bind();
            self.texture.bind(0);

            let x = f32::sin(self.test_val);
            self.test_val += 0.0004;

            self.batch.begin_batch();
            self.batch.draw(-0.25 - x, 0.0, 0.5, 0.5);
            //self.batch.draw(x, 0.5, 0.1, 0.1);
            self.batch.end_batch();

            //gl::BindVertexArray(self.vao);

            //gl::DrawArrays(gl::TRIANGLES, 0, 3);
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
