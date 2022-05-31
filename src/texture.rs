extern crate glium;
extern crate image;

use glium::{
	Display,
	texture::{
		SrgbTexture2d,
		RawImage2d
	}
};

use image::io::Reader as ImageReader;

pub struct Texture {
	pub texture: SrgbTexture2d
}

impl Texture {
	pub fn load(display: &Display, path: &str) -> Result<Self, String> {
		let reader = match ImageReader::open(path.to_string()) {
			Err(_) => return Err("cannot read file".to_string()),
			Ok(reader) => reader,
		};
		let img = match reader.decode() {
			Err(_) => return Err("cannot decode image".to_string()),
			Ok(img) => img,
	 	};
		let img = img.to_rgba8();

		let image_dimensions = img.dimensions();
		let image = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), image_dimensions);

		let texture = match SrgbTexture2d::new(display, image) {
			Err(_) => return Err("cannot create Srgb texture".to_string()),
			Ok(tex) => tex,
	 	};

		Ok(Self {
			texture
		})
	}
}