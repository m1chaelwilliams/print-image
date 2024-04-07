# Print Image
A super simple solution to printing images in the terminal

# Features
- Printing images
- Resizing output
- Custom pixel handling

# Usage

```rust
use print_image;
use std::path::Path;

fn main() -> Result<(), print_image::PrintImageError> {
	print_img(
		Path::new("path/to/file.png"), 
		Some((12, 12)), 
		print_image::handle_pixel_rgb
	)
}
```

# Custom Pixel Handler

```rust
use print_image;
use std::path::Path;

fn main() -> Result<(), print_image::PrintImageError> {
	print_img(
		Path::new("path/to/file.png"), 
		Some((12, 12)), 
		|pixel| {
			match pixel[0] {
				0..=128 => "+".to_string(),
				129..=255 => "-".to_string()
			}
		}
	)
}
```

# Dependencies
- [image](https://github.com/image-rs/image) - Licensed under MIT or Apache 2.0
- [colored](https://docs.rs/colored/latest/colored/) - Licensed under Mozilla Public License Version 2.0

99.99% of the legwork was done by the dependencies for this project. Print Image only provides an easy to use wrapper over them.

# License
There is no license for this code. You can do whatever you want with it. However, if you find a bug, it would be awesome if you let me know!