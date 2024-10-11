extern crate image;
use image::{DynamicImage, GenericImageView, ImageReader};

pub struct Texture {
    image: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path).unwrap().decode().unwrap();
        let width = img.width();
        let height = img.height();
        Texture { image: img, width, height }
    }

    pub fn get_pixel_color(&self, x: u32, y: u32) -> [u8; 4] {
        let pixel = self.image.get_pixel(x, y).0;
        [pixel[0], pixel[1], pixel[2], 255] // Considera alfa como 255 (opaco)
    }
}
