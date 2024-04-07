use image_to_terminal::printimage::{print_img, ImageCache, ImageCacheError};
use std::path::Path;

fn main() -> Result<(), ImageCacheError> {
    
    let cache = ImageCache::create_scaled(
        Path::new("assets/fireflower.png"),
        (0.01, 0.01)
        )?;    

    print_img(&cache);

    Ok(())
}