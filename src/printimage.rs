// PrintImage is a convenience small library for printing an image in the terminal
// 
// License
//
// MIT License

// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:

// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

//! # Print Image
//! 
//! Print an image to the terminal!

use std::path::Path;
use image::{self, GenericImageView, Rgba};
use colored::{self, Colorize};
use image::imageops::FilterType;

/// Error enum for handling image loading errors
/// 
/// # Example
/// 
/// ```rust
/// let _ = match print_img(path, None, handle_pixel_rgb) {
/// 	Ok(val) => val,
/// 	Err(e) => return Err(e);
/// }
/// ```
#[derive (Debug)]
pub enum PrintImageError {
	/// Wrapper over `image::ImageError` from the [image](https://github.com/image-rs/image) crate.
	ImageLoadError(image::ImageError)
}

/// *Private Function*: Helper function for loading images
/// 
/// # Example
/// 
/// ```rust
/// let img = img_load(path)?;
/// ```
fn img_load(path: &Path) -> Result<image::DynamicImage, PrintImageError> {
	match image::open(path) {
		Ok(result) => Ok(result),
		Err(e) => Err(PrintImageError::ImageLoadError(e))
	}
}

/// A `handle_pixel` function that returns a colored block to the screen
/// 
/// # Usage
/// 
/// ```rust
/// let _ = print_img(path, None, handle_pixel_rgb);
/// ```
pub fn handle_pixel_rgb(pixel: Rgba<u8>) -> String {
	let color = colored::Color::TrueColor {
		r: pixel[0],
		g: pixel[1],
		b: pixel[2]
	};
	
	format!("{}", "██".color(color))
}

/// A `handle_pixel` function that returns a colored block to the screen
/// 
/// # Usage
/// 
/// ```rust
/// let _ = print_img(path, None, handle_pixel_ascii);
/// ```
pub fn handle_pixel_ascii(pixel: Rgba<u8>) -> String {
	let sum: u32 = pixel[0] as u32 + pixel[1] as u32 + pixel[1] as u32;

	let character = match sum {
		0..=85 => " ",
		86..=170 => ":",
		171..=255 => ";",
		256..=340 => "=",
		341..=425 => "*",
		426..=510 => "#",
		511..=595 => "@",
		596..=680 => "&",
		681..=765 => "%",
		_ => ""
	};

	let result = format!("{:width$}", character, width = 1);

	result
}

/// Prints the image to the screen at `size` or default of `None`
/// Uses `handle_pixel` closure for custom pixel handling (with useful defaults)
/// 
/// ### Default Pixel Handlers:
/// - `handle_pixel_rgb`
/// - `handle_pixel_ascii`
/// 
/// # Usage
/// 
/// ```rust
/// let _ = print_img(path, Some((width, height)), |pixel| {
/// 	if pixel[0] > 128 {
/// 		return "-".to_string();
/// 	}
/// 	"+".to_string()
/// })
/// ```
pub fn print_img<F>(path: &Path, size: Option<(u32, u32)>, handle_pixel: F) -> Result<(), PrintImageError> 
where
	F: Fn(image::Rgba<u8>) -> String
{
	let mut img = img_load(path)?;
	
	if let Some(size) = size {
		img = img.resize(size.0, size.1, FilterType::CatmullRom);
	}

	let (w, h) = img.dimensions();

	for y in 0..h {
		for x in 0..w {
			let pixel = img.get_pixel(x, y);
			print!("{}", handle_pixel(pixel));
		}
		println!();
	}
	
	Ok(())
}