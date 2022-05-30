pub fn create_matrix(v: (f32, f32, f32), s: f32) -> [[f32; 4]; 4] {
	let a = v.0;
	let b = v.1;
	let y = v.2;
	[
		[
			s * (b.cos() * y.cos()),
			s * (a.sin() * b.sin() * y.cos() - a.cos() * y.sin()),
			s * (a.cos() * b.sin() * y.cos() + a.sin() * y.sin()),
			0.0
		],
		[
			s * (b.cos() * y.sin()),
			s * (a.sin() * b.sin() * y.sin() + a.cos() * y.cos()),
			s * (a.cos() * b.sin() * y.sin() - a.sin() * y.cos()),
			0.0
		],
		[
			s * (-b.sin()),
			s * (a.sin() * b.cos()),
			s * (a.cos() * b.cos()),
			0.0
		],
		[0.0, 0.0, 0.0, 1.0]
	]
}
