extern crate image;

use std::path::Path;

use cgmath::Vector3;

// Refer: https://github.com/ranveeraggarwal/rust-raytracer/blob/master/src/io/write.rs
pub fn write_img(img: &[Vec<Vector3<f64>>], filename: &str) {
    let path = Path::new(&filename);
    let display = path.display();
    let size_y = img.len() as u32;
    let size_x = img[0].len() as u32;

    let mut imgbuf = image::ImageBuffer::new(size_x, size_y);

    for (y, row) in img.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            imgbuf.put_pixel(
                x as u32,
                y as u32,
                image::Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]),
            );
        }
    }

    let _ = image::ImageRgb8(imgbuf).save(&path);
    println!("Successfully wrote to {}", display);
}
