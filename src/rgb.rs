pub fn from_hsl(h: f32, s: f32, l: f32) -> [f32; 3]
{
	let m2 =
		if l <= 0.5 { l * (s + 1.0) }
		else { l + s - l * s };
	let m1 = l * 2.0 - m2;

	let htr = |m1: f32, m2: f32, mut h: f32| {
		if h < 0.0 { h = h + 1.0 }
		if h > 1.0 { h = h - 1.0 }

		if h * 6.0 < 1.0 { m1 + (m2 - m1) * h * 6.0 }
		else if h * 2.0 < 1.0 { m2 }
		else if h * 3.0 < 2.0 { m1 + (m2 - m1) * (2.0 / 3.0 - h) * 6.0 }
		else { m1 }
	};

	let r = htr(m1, m2, h + 1.0 / 3.0);
	let g = htr(m1, m2, h              );
	let b = htr(m1, m2, h - 1.0 / 3.0);

	[r, g, b]
}