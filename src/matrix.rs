pub fn create_matrix(v: (f32, f32, f32)) -> [[f32; 4]; 4] {
	let a = v.0;
	let b = v.1;
	let y = v.2;
	[
		[
			(b.cos() * y.cos()),
			(a.sin() * b.sin() * y.cos() - a.cos() * y.sin()),
			(a.cos() * b.sin() * y.cos() + a.sin() * y.sin()),
			0.0
		],
		[
			(b.cos() * y.sin()),
			(a.sin() * b.sin() * y.sin() + a.cos() * y.cos()),
			(a.cos() * b.sin() * y.sin() - a.sin() * y.cos()),
			0.0
		],
		[
			(-b.sin()),
			(a.sin() * b.cos()),
			(a.cos() * b.cos()),
			0.0
		],
		[0.0, 0.0, 0.0, 1.0]
	]
}

pub fn get_perspective(width: f32, height: f32) -> [[f32; 4]; 4] {
	let max = f32::max(width, height);

    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1_048_576.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [ f * (height / max),  0.0               ,  0.0                           ,  0.0],
        [ 0.0               ,  f * (width / max) ,  0.0                           ,  0.0],
        [ 0.0               ,  0.0               ,  (zfar+znear)/(zfar-znear)     ,  1.0],
        [ 0.0               ,  0.0               ,  -(2.0*zfar*znear)/(zfar-znear),  0.0],
    ]
}