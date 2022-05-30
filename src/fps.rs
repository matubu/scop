use std::time::Instant;
use std::io::{Write, stdout};

pub struct Fps {
	hist: Vec<f64>,
	last: Instant
}

impl Fps {
	pub fn new() -> Self {
		Self {
			hist: Vec::new(),
			last: Instant::now()
		}
	}
	pub fn tick(&mut self) -> f64 {
		let dt = self.last.elapsed().as_secs_f64();
		self.last = Instant::now();
		
		self.hist.push(1.0 / dt);
		if self.hist.len() > 32 {
			self.hist.remove(0);
		}

		let mut fps = 0.0;
		let mut div = 0.0;
		for i in 0..self.hist.len() {
			let fac = 1.0 + i as f64 / 16.0;
			div += fac;
			fps += self.hist[i] * fac;
		}

		print!("\r\x1B[2K{:.1} \x1B[90mfps\x1B[0m", fps / div);
		stdout().flush().unwrap();

		dt
	}
}