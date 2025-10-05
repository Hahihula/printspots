use image::{ImageBuffer, Rgb, RgbImage};

use crate::grayscale::ColorPalette;

pub fn dither_to_palette(input_image: &RgbImage, palette: &ColorPalette) -> RgbImage {
    let (width, height) = input_image.dimensions();
    let mut grayscale = to_grayscale(input_image);
    let mut output = input_image.clone();
    
    // Floyd-Steinberg dithering
    for y in 0..height {
        for x in 0..width {
            let old_pixel = *grayscale.get_pixel(x, y);
            let new_pixel = find_closest_palette_color(&old_pixel, palette);
            output.put_pixel(x, y, new_pixel);
            
            let error = calculate_color_error(&old_pixel, &new_pixel);
            distribute_error(&mut grayscale, x, y, width, height, &error);
        }
    }
    
    output
}

pub fn to_grayscale(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut grayscale = ImageBuffer::new(width, height);
    
    for (x, y, pixel) in image.enumerate_pixels() {
        let gray = (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
        grayscale.put_pixel(x, y, Rgb([gray, gray, gray]));
    }
    
    grayscale
}

pub fn mirror_image(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut mirrored_image = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let p = image.get_pixel(x, y);
            mirrored_image.put_pixel(width - 1 - x, y, *p);
        }
    }
    mirrored_image
}

fn find_closest_palette_color(pixel: &Rgb<u8>, palette: &ColorPalette) -> Rgb<u8> {
    palette.colors.iter()
        .min_by(|a, b| {
            color_distance(pixel, a).partial_cmp(&color_distance(pixel, b)).unwrap()
        })
        .copied()
        .unwrap_or(palette.colors[0])
}

fn color_distance(c1: &Rgb<u8>, c2: &Rgb<u8>) -> f32 {
    let dr = c1[0] as f32 - c2[0] as f32;
    let dg = c1[1] as f32 - c2[1] as f32;
    let db = c1[2] as f32 - c2[2] as f32;
    (dr * dr + dg * dg + db * db).sqrt()
}

fn calculate_color_error(old: &Rgb<u8>, new: &Rgb<u8>) -> [i16; 3] {
    [
        old[0] as i16 - new[0] as i16,
        old[1] as i16 - new[1] as i16,
        old[2] as i16 - new[2] as i16,
    ]
}

fn distribute_error(image: &mut RgbImage, x: u32, y: u32, width: u32, height: u32, error: &[i16; 3]) {
    let mut distribute = |dx: i32, dy: i32, factor: f32| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        
        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
            let pixel = image.get_pixel_mut(nx as u32, ny as u32);
            for i in 0..3 {
                pixel[i] = ((pixel[i] as i16 + (error[i] as f32 * factor) as i16).clamp(0, 255)) as u8;
            }
        }
    };
    
    distribute(1, 0, 7.0/16.0);   // Right
    distribute(-1, 1, 3.0/16.0);  // Bottom-left
    distribute(0, 1, 5.0/16.0);   // Bottom
    distribute(1, 1, 1.0/16.0);   // Bottom-right
}