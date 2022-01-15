fn render(delta_time: f32) {
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn main() {
    let config = rlib::RlibConfig {
        window_title: "Rlib Test".to_string(),
        window_width: 1600,
        window_height: 900,
        render_func: render,
    };
    rlib::init_sdl(config);
}
