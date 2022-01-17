#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;

extern crate sdl2;

extern crate gl;

pub mod gfx;

pub trait RLibApp {
	fn new() -> Self;
	fn init(&mut self) -> ();
	fn render(&mut self) -> ();
}

pub struct RlibConfig {
	pub window_title: String,
	pub window_width: u32,
	pub window_height: u32,
}

impl Default for RlibConfig {
	fn default() -> RlibConfig {
		RlibConfig {
			window_title: "Title".to_string(),
			window_width: 800,
			window_height: 600,
		}
	}
}

pub fn init<App: RLibApp>(config: RlibConfig) {
	TermLogger::init(
		LevelFilter::max(),
		Config::default(),
		TerminalMode::Mixed,
		ColorChoice::Auto,
	)
	.unwrap();

	info!("Initializing rlib");
	info!("Initializing SDL");
	let sdl = sdl2::init().unwrap();
	let sdl_video = sdl.video().unwrap();

	let gl_attr = sdl_video.gl_attr();
	gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
	gl_attr.set_context_version(4, 5);

	let window = sdl_video
		.window(
			&config.window_title,
			config.window_width,
			config.window_height,
		)
		.opengl()
		.resizable()
		.build()
		.unwrap();

	let _gl_context = window.gl_create_context().unwrap();
	let _gl = gl::load_with(|s| sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

	let mut app = App::new();
	app.init();

	let mut window_width: i32 = config.window_width as i32;
	let mut window_height: i32 = config.window_height as i32;

	let mut event_pump = sdl.event_pump().unwrap();
	'main_loop: loop {
		for event in event_pump.poll_iter() {
			match event {
				sdl2::event::Event::Quit { .. } => break 'main_loop,
				sdl2::event::Event::Window {
					timestamp: _,
					window_id: _,
					win_event,
				} => match win_event {
					sdl2::event::WindowEvent::Resized(w, h) => {
						window_height = h;
						window_width = w;
					}
					_ => {}
				},
				_ => {}
			}
		}

		unsafe {
			gl::Viewport(
				0,
				0,
				i32::try_from(window_width).unwrap(),
				i32::try_from(window_height).unwrap(),
			);
		}

		app.render();

		window.gl_swap_window();
	}
}
