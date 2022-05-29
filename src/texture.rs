extern crate image;
extern crate glium;

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
	pub fn load(display: &Display, path: &String) -> Self {
		let img = ImageReader::open(path).unwrap().decode().unwrap().to_rgba8();

		let image_dimensions = img.dimensions();
		let image = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), image_dimensions);
		
		Self {
			texture: SrgbTexture2d::new(display, image).unwrap()
		}
	}
}