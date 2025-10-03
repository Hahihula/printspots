use serde::{Serialize, Deserialize};

/// Global configuration for the entire printing process
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintConfig {
    pub base_thickness: f32,
    pub layer_thickness: f32,
    pub image_size_mm: f32,
    pub max_layers: f32,
}

impl Default for PrintConfig {
    fn default() -> Self {
        Self {
            base_thickness: 1.0,
            layer_thickness: 0.05,
            image_size_mm: 100.0,
            max_layers: 19.0,
        }
    }
    
}

impl PrintConfig {
    pub fn new(base_thickness: f32, layer_thickness: f32, image_size_mm: f32, max_layers: f32) -> Self {
        Self {
            base_thickness,
            layer_thickness,
            image_size_mm,
            max_layers,
        }
    }

    pub fn calculate_black_thickness(&self, white_layers: u32, flat_top: bool) -> f32 {
        if flat_top {
            self.base_thickness + (self.max_layers * self.layer_thickness) - (white_layers as f32 * self.layer_thickness)
        } else {
            self.base_thickness
        }
    }

    pub fn calculate_total_height(&self, white_layers: u32, flat_top: bool) -> f32 {
        if flat_top {
            self.base_thickness + (self.max_layers * self.layer_thickness)
        } else {
            self.base_thickness + (white_layers as f32 * self.layer_thickness)
        }
    }

    pub fn pixel_size(&self, image_width_pixels: u32, image_height_pixels: u32) -> (f32,f32) {
        if image_width_pixels > image_height_pixels {
            // Width is the limiting dimension
            let pixel_width = self.image_size_mm / image_width_pixels as f32;
            let pixel_height = pixel_width; // Square pixels
            (pixel_width, pixel_height)
        } else {
            // Height is the limiting dimension  
            let pixel_height = self.image_size_mm / image_height_pixels as f32;
            let pixel_width = pixel_height; // Square pixels
            (pixel_width, pixel_height)
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrintingConstraints {
    pub min_feature_size_mm: f32,
    pub merge_small_features: bool,
    pub erosion_dilation_passes: u32,
}

impl Default for PrintingConstraints {
    fn default() -> Self {
        Self {
            min_feature_size_mm: 0.5,
            merge_small_features: true,
            erosion_dilation_passes: 1,
        }
    }
}

impl PrintingConstraints {
    pub fn calculate_min_pixels(&self, config: &PrintConfig, image_width: u32, image_height: u32) -> (u32, u32) {
        let (pixel_width, pixel_height) = config.pixel_size(image_width, image_height);
        let pixels_per_mm_x = 1.0 / pixel_width;
        let pixels_per_mm_y = 1.0 / pixel_height;
        
        let min_pixels_x = (self.min_feature_size_mm * pixels_per_mm_x).ceil() as u32;
        let min_pixels_y = (self.min_feature_size_mm * pixels_per_mm_y).ceil() as u32;
        
        (min_pixels_x, min_pixels_y)
    }
}