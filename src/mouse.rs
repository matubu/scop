extern crate glium;

use glium::glutin::event::{Event, WindowEvent, MouseButton, ElementState};

pub struct Mouse {
	button: MouseButton,
	pos: Option<(f64, f64)>,
	delta: (f64, f64)
}

impl Mouse {
	pub fn new() -> Self {
		Self {
			button: MouseButton::Other(0),
			pos: None,
			delta: (0.0, 0.0)
		}
	}

	pub fn apply_events(&mut self, events: &Vec<Event<()>>) {
		let old_pos: Option<(f64, f64)> = self.pos;
		self.delta = (0.0, 0.0);
		for event in events {
			match event {
				Event::Suspended
				| Event::WindowEvent {
					event: WindowEvent::CursorLeft { .. },
					..
				} => self.pos = None,

				Event::WindowEvent {
					event: WindowEvent::MouseInput {
						state: ElementState::Released,
						..
					},
					..
				} => self.button = MouseButton::Other(0),

				Event::WindowEvent {
					event: WindowEvent::MouseInput {
						state: ElementState::Pressed,
						button,
						..
					},
					..
				} => self.button = *button,

				Event::WindowEvent {
					event: WindowEvent::CursorMoved {
						position,
						..
					},
					..
				} => {
					self.delta = match old_pos {
						Some(old) => (position.x - old.0, position.y - old.1),
						None => (0.0, 0.0)
					};
					self.pos = Some((position.x, position.y))
				},

				_ => ()
			}
		}
	}

	pub fn get_delta(&self) -> (f64, f64) {
		self.delta
	}
	pub fn is_left_button_pressed(&self) -> bool {
		self.button == MouseButton::Left
	}
}