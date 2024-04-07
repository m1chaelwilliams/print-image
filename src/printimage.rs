use std::path::Path;
use image::{self, DynamicImage, GenericImageView};
use colored::{self, Colorize};
use std::ops::Range;
use crate::rgb::RGB;

// struct used *only* for calculating averages
// NOT a valid RGB value. It can be outside of 0-255
#[derive (Default)]
struct RGB64(u64, u64, u64);

#[derive (Debug)]
pub enum ImageCacheError {
	ScaleInvalid(String),
	MismatchedSize(String),
	ImageLoadError(image::ImageError)
}

fn img_load(path: &Path) -> Result<image::DynamicImage, ImageCacheError> {
	match image::open(path) {
		Ok(result) => Ok(result),
		Err(e) => Err(ImageCacheError::ImageLoadError(e))
	}
}

fn calc_avg_color(img: &DynamicImage, w_range: Range<u32>, h_range: Range<u32>) -> RGB {
	let mut result: RGB = RGB::default();
	let mut num_pixels = 0;

	let mut average = RGB64::default();

	for x in w_range.clone() {
		for y in h_range.clone() {
			let pixel = img.get_pixel(x, y);

			average.0 += pixel[0] as u64;
			average.1 += pixel[1] as u64;
			average.2 += pixel[2] as u64;
			
			num_pixels += 1;
		}
	}

	result.r = (average.0 / num_pixels) as u8;
	result.g = (average.1 / num_pixels) as u8;
	result.b = (average.2 / num_pixels) as u8;

	result
}

// caches scaled image data as a Vec<Vec<RGB>>.
// Does NOT store original data. If scale factor is < 1, data will be lost.
pub struct ImageCache {
	pub pixels: Vec<Vec<RGB>>,
}

impl ImageCache {
	pub fn height(&self) -> usize {
		self.pixels.len()
	}

	pub fn width(&self) -> usize {
		if self.height() > 0 {
			return self.pixels[0].len();
		}
		0
	}

	pub fn from_image_scaled(img: &DynamicImage, scale: (f32, f32)) -> Result<Self, ImageCacheError> {
		if scale.0 <= 0.0 || scale.1 <= 0.0 {
			return Err(ImageCacheError::ScaleInvalid("Scale must be positive".into()));
		}

		let (width, height) = img.dimensions();

		let mut pixels: Vec<Vec<RGB>> = Vec::new();

		let w_step = width / (width as f32 * scale.0) as u32;
		let h_step = width / (height as f32 * scale.1) as u32;
		
		for y in (0..height).step_by(h_step as usize) {
			let mut row: Vec<RGB> = Vec::new();
			for x in (0..width).step_by(w_step as usize) {
				let avg_color = calc_avg_color(&img, x..x+w_step, y..y+h_step);
				row.push(avg_color);
			}
			pixels.push(row);
		}

		Ok(Self {
			pixels
		})
	}

	pub fn create_scaled(path: &Path, scale: (f32, f32)) -> Result<Self, ImageCacheError> {
		let img = img_load(path)?;

		Self::from_image_scaled(&img, scale)
	}

	pub fn create(path: &Path) -> Result<Self, ImageCacheError> {
		let img = img_load(path)?;

		let mut pixels = Vec::new();

		let (width, height) = img.dimensions();

		for y in 0..height {
			let mut row = Vec::new();
			for x in 0..width {
				let pixel = img.get_pixel(x, y);
				let color = RGB::new(pixel[0], pixel[1], pixel[2]);

				row.push(color);
			}
			pixels.push(row);
		}

		Ok(Self {
			pixels
		})
	}
}

pub fn print_img(img_cache: &ImageCache) {
	for row in img_cache.pixels.iter() {
		for pixel in row.iter() {
			let color = colored::Color::TrueColor {
				r: pixel.r,
				g: pixel.g,
				b: pixel.b,
			};

			print!("{}", "█".color(color));
		}
		println!();
	}
}