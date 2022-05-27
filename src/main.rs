extern crate glium;

use glium::{
	{Display, Surface, Program},
	glutin::{
		{ContextBuilder, Api, GlRequest},
		event::{Event, WindowEvent},
		event_loop::{EventLoop, ControlFlow},
		window::{WindowBuilder, Fullscreen},
		dpi::{LogicalSize}
	}
};

use std::time::{Instant};

const PARTICULE_COUNT: u32 = 100_000;

fn main() {
	let event_loop = EventLoop::new();
	let win = WindowBuilder::new()
				.with_title("particule-system")
				.with_resizable(false);
	let ctx = ContextBuilder::new()
				.with_multisampling(0)
				.with_vsync(true)
				.with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)));
	let display = Display::new(win, ctx, &event_loop).unwrap();

	let primary_monitor = display.gl_window().window().primary_monitor().unwrap();
	let fullscreen = Fullscreen::Borderless(Some(primary_monitor));
	display.gl_window().window().set_fullscreen(Some(fullscreen));

	// struct Point {
	// 	pub pos: [f32; 2],
	// 	pub vel: [f32; 2]
	// }
	// let mut vertex_buffer = glium::buffer::Buffer::<[Point]>
	// 	::empty_unsized(
	// 		&display,
	// 		BufferType::UniformBuffer,
	// 		size_of::<[Point]>() * 8,
	// 		BufferMode::Default
	// 	)
	// 	.unwrap();
	// vertex_buffer.map()[0].x = 100;
	// vertex_buffer.map()[0].y = 100;

	let mut previous = Instant::now();
	event_loop.run(move |event, _, control_flow| {
		// display.set_cursor(MouseCursor::Crosshair);

		let now = Instant::now();
		let dt = now.duration_since(previous).as_secs_f32();
		previous = now;

		println!("{dt}s");

		let mut target = display.draw();
		target.clear_color(0.0, 0.0, 0.02, 1.0);
		target.finish().unwrap();

		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => *control_flow = ControlFlow::Exit,
			_ => ()
		}
	});
}
