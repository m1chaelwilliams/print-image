use print_image::*;
use std::path::Path;

fn main() -> Result<(), PrintImageError> {    

    let _ = print_img(Path::new("assets/fireflower.png"), Some((12, 12)), handle_pixel_rgb);

    Ok(())
}