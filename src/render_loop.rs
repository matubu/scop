extern crate glium;

use glium::glutin::{
	event::{Event, WindowEvent, StartCause},
	event_loop::{EventLoop, ControlFlow},
};

use std::time::{Duration, Instant};

pub fn render_loop_start<Func>(
	event_loop: EventLoop<()>,
	mut render_callback: Func
)
where
	Func: 'static + FnMut(&Vec<Event<()>>),
{
	let mut events_buffer = Vec::new();
	let mut next_frame_time = Instant::now();

	event_loop.run(move |event, _, control_flow| {
		let exit = match event.to_static() {
			Some(Event::NewEvents(cause)) => {
				match cause {
					StartCause::ResumeTimeReached { .. } | StartCause::Init => {
						next_frame_time = Instant::now() + Duration::from_millis(16);
						render_callback(&events_buffer);
						events_buffer.clear();
						false
					},
					_ => false
				}
			},
			Some(event) => {
				match event {
					Event::WindowEvent {
						event: WindowEvent::CloseRequested,
						..
					} => true,
					_ => {
						events_buffer.push(event);
						false
					}
				}
			}
			None => false,
		};

		if exit {
			*control_flow = ControlFlow::Exit;
		}
		else {
			*control_flow = ControlFlow::WaitUntil(next_frame_time);
		}
	})
}