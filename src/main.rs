extern crate sdl2;
use sdl2::event::Event;
use sdl2::video::GLProfile;

extern crate gl;

mod shader;
use shader::Shader;

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let gl_attr = video_subsystem.gl_attr();
	gl_attr.set_context_profile(GLProfile::Core);
	gl_attr.set_context_version(4, 1);

	let window = video_subsystem
		.window("particule_system", 900, 700)
		.opengl()
		.resizable()
		.build()
		.unwrap();

	let ctx = window.gl_create_context().unwrap();
	gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

	let mut event_pump = sdl_context.event_pump().unwrap();
	'event_loop: loop {
		unsafe {
			gl::Viewport(0, 0, 900, 700); // set viewport
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
		}
		
		window.gl_swap_window();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => break 'event_loop,
				_ => {},
			}
		}
	}
}
