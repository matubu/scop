use std::ops;

pub struct Vector {
	d: [f32; 3]
}

impl Vector {
	pub fn new() -> Self {
		Self {
			d: [0.0, 0.0, 0.0]
		}
	}
	pub fn from_vec(vec: &Vec<f32>) -> Self {
		Self {
			d: vec.as_slice().try_into().unwrap()
		}
	}
	pub fn to_array(&self) -> [f32; 3] {
		self.d
	}
	pub fn len(&self) -> f32 {
		(
			self.d[0].powi(2)
			+ self.d[1].powi(2)
			+ self.d[2].powi(2)
		).sqrt()
	}
	pub fn x(&self) -> f32 {
		self.d[0]
	}
	pub fn y(&self) -> f32 {
		self.d[1]
	}
	pub fn z(&self) -> f32 {
		self.d[2]
	}
	pub fn cross(&self, other: &Self) -> Self {
		Self {
			d: [
				self.y() * other.z() - self.z() * other.y(),
				self.z() * other.x() - self.x() * other.z(),
				self.x() * other.y() - self.y() * other.x()
			]
		}
	}
	pub fn normalize(&self) -> Self {
		self / self.len()
	}
}

impl ops::Div<f32> for &Vector {
	type Output = Vector;

	fn div(self, other: f32) -> Vector {
		Vector {
			d: [
				self.d[0] / other,
				self.d[1] / other,
				self.d[2] / other
			]
		}
	}
}

impl ops::Sub<&Vector> for &Vector {
	type Output = Vector;

	fn sub(self, other: &Vector) -> Vector {
		Vector {
			d: [
				self.d[0] - other.d[0],
				self.d[1] - other.d[1],
				self.d[2] - other.d[2]
			]
		}
	}
}