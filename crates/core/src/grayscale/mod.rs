use std::{collections::HashMap, fs::{self, File}, io::{self, BufWriter, Read, Write}, path::Path};
use image::{Rgb, RgbImage};
use serde::{Deserialize, Serialize};
use threemf::model::{Base, BaseMaterials, Item, Model, Object};


use crate::{config::{PrintConfig, PrintingConstraints}, mesh::calculate_normal, utils::PrintObjects};

pub mod calibration;
pub mod image_processing;
pub mod generate;

// Helper struct for serializing/deserializing Rgb<u8>
#[derive(Serialize, Deserialize)]
struct SerializableRgb(#[serde(with = "serde_bytes")] pub [u8; 3]);

impl From<Rgb<u8>> for SerializableRgb {
    fn from(rgb: Rgb<u8>) -> Self {
        SerializableRgb(rgb.0)
    }
}

impl From<SerializableRgb> for Rgb<u8> {
    fn from(s_rgb: SerializableRgb) -> Self {
        Rgb(s_rgb.0)
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)] // Add Serialize, Deserialize, PartialEq
pub struct ColorPalette {
    #[serde(with = "vec_rgb_serde")] // Use a custom module for Vec<Rgb<u8>>
    pub colors: Vec<Rgb<u8>>,
    pub layer_counts: Vec<u32>,
}

// Custom serialization/deserialization for Vec<Rgb<u8>>
mod vec_rgb_serde {
    use super::{Rgb, SerializableRgb};
    use serde::{Serializer, Deserializer, Serialize, Deserialize};
    use serde::ser::Error as SerError;
    use serde::de::Error as DeError;

    pub fn serialize<S>(colors: &Vec<Rgb<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serializable_colors: Vec<SerializableRgb> = colors.iter().cloned().map(SerializableRgb::from).collect();
        serializable_colors.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Rgb<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serializable_colors = Vec::<SerializableRgb>::deserialize(deserializer)?;
        Ok(serializable_colors.into_iter().map(Rgb::from).collect())
    }
}


impl ColorPalette {
    pub fn fake(max_layers: u32) -> Self {
        let mut colors = Vec::new();
        let mut layer_counts = Vec::new();
        
        for i in 0..=max_layers {
            let intensity = (255.0 * i as f32 / max_layers as f32) as u8;
            colors.push(Rgb([intensity, intensity, intensity]));
            layer_counts.push(i);
        }
        
        Self { colors, layer_counts }
    }

    pub fn get_layer_count_for_color(&self, color: &Rgb<u8>) -> u32 {
        self.colors.iter()
            .zip(self.layer_counts.iter())
            .find(|(c, _)| **c == *color)
            .map(|(_, &count)| count)
            .unwrap_or(0)
    }

    pub fn get_color_for_layer_count(&self, layer_count: u32) -> Option<Rgb<u8>> {
        self.layer_counts.iter()
            .zip(self.colors.iter())
            .find(|&(&count, _)| count == layer_count)
            .map(|(_, &color)| color)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let toml_string = toml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to serialize ColorPalette to TOML: {}", e)))?;
        
        let mut file = fs::File::create(path)?;
        file.write_all(toml_string.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let palette: ColorPalette = toml::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to deserialize ColorPalette from TOML: {}", e)))?;
        Ok(palette)
    }
}

pub fn export_to_stl(objects: &PrintObjects, filename_black: &str, filename_white: &str) -> io::Result<()> {
    let mut file = File::create(filename_black)?;
    writeln!(file, "solid object")?;
    
    for triangle in &objects.black_mesh.triangles.triangle {
        let v0 = &objects.black_mesh.vertices.vertex[triangle.v1];
        let v1 = &objects.black_mesh.vertices.vertex[triangle.v2];
        let v2 = &objects.black_mesh.vertices.vertex[triangle.v3];
        let normal = calculate_normal(v0, v1, v2);
        
        writeln!(file, "  facet normal {} {} {}", normal.x, normal.y, normal.z)?;
        writeln!(file, "    outer loop")?;
        writeln!(file, "      vertex {} {} {}", v0.x, v0.y, v0.z)?;
        writeln!(file, "      vertex {} {} {}", v1.x, v1.y, v1.z)?;
        writeln!(file, "      vertex {} {} {}", v2.x, v2.y, v2.z)?;
        writeln!(file, "    endloop")?;
        writeln!(file, "  endfacet")?;
    }
    writeln!(file, "endsolid object")?;

    let mut filew = File::create(filename_white)?;
    writeln!(filew, "solid object")?;
    
    for triangle in &objects.white_mesh.triangles.triangle {
        let v0 = &objects.white_mesh.vertices.vertex[triangle.v1];
        let v1 = &objects.white_mesh.vertices.vertex[triangle.v2];
        let v2 = &objects.white_mesh.vertices.vertex[triangle.v3];
        let normal = calculate_normal(v0, v1, v2);
        
        writeln!(filew, "  facet normal {} {} {}", normal.x, normal.y, normal.z)?;
        writeln!(filew, "    outer loop")?;
        writeln!(filew, "      vertex {} {} {}", v0.x, v0.y, v0.z)?;
        writeln!(filew, "      vertex {} {} {}", v1.x, v1.y, v1.z)?;
        writeln!(filew, "      vertex {} {} {}", v2.x, v2.y, v2.z)?;
        writeln!(filew, "    endloop")?;
        writeln!(filew, "  endfacet")?;
    }
    
    writeln!(filew, "endsolid object")?;
    Ok(())
}


pub fn export_to_3mf(objects: &PrintObjects, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut model = Model::default();
    model.unit = threemf::model::Unit::Millimeter;
    let mut object_id = 1;

    // Define materials/colors
    let materials = BaseMaterials {
        id: 1,
        base: vec![
            Base {
                name: "Black".to_string(),
                displaycolor: "#000000".to_string(),
            },
            Base {
                name: "White".to_string(),
                displaycolor: "#FFFFFF".to_string(),
            },
        ],
    };

    model.resources.basematerials = Some(vec![materials]);
    
    // Add black mesh as object 1
    if !objects.black_mesh.vertices.vertex.is_empty() {
        let object = Object {
            id: object_id,
            mesh: Some(threemf::Mesh {
                vertices: objects.black_mesh.vertices.clone(),
                triangles: objects.black_mesh.triangles.clone(),
            }),
            name: Some("Black Layer".to_string()),
            partnumber: None,
            pid: Some(1),           // Reference to basematerials group
            pindex: Some(0),        // First material (Black)
            components: None,
        };
        
        model.resources.object.push(object);
        object_id += 1;
    }
    
    let black_id = if !objects.black_mesh.vertices.vertex.is_empty() { object_id - 1 } else { 0 };
    
    // Add white mesh as object 2
    if !objects.white_mesh.vertices.vertex.is_empty() {
        let object = Object {
            id: object_id,
            mesh: Some(threemf::Mesh {
                vertices: objects.white_mesh.vertices.clone(),
                triangles: objects.white_mesh.triangles.clone(),
            }),
            name: Some("White Layer".to_string()),
            partnumber: None,
            pid: Some(1),           // Reference to basematerials group
            pindex: Some(1),        // Second material (White)
            components: None,
        };
        
        model.resources.object.push(object);
        object_id += 1;
    }
    
    let white_id = if !objects.white_mesh.vertices.vertex.is_empty() { object_id - 1 } else { 0 };
    
    // Create parent object with components referencing both meshes
    if black_id > 0 || white_id > 0 {
        let mut component_vec = Vec::new();
        
        if black_id > 0 {
            component_vec.push(threemf::model::Component {
                objectid: black_id,
                transform: None,
            });
        }
        
        if white_id > 0 {
            component_vec.push(threemf::model::Component {
                objectid: white_id,
                transform: None,
            });
        }
        
        let parent_object = Object {
            id: object_id,
            mesh: None,
            name: Some("Print Object".to_string()),
            partnumber: None,
            pid: None,
            pindex: None,
            components: Some(threemf::model::Components {
                component: component_vec,
            }),
        };
        
        model.resources.object.push(parent_object);
        
        // Add ONLY the parent object to the build
        model.build.item.push(Item {
            objectid: object_id,
            transform: None,
            partnumber: None,
        });
    }
    
    // Write the model to file
    let file = File::create(filename)?;
    threemf::write(&mut BufWriter::new(file), model)?;
    
    Ok(())
}

/// Improve printability of image
/// 



pub fn enforce_min_feature_size(
    dithered_image: &RgbImage,
    palette: &ColorPalette,
    constraints: &PrintingConstraints,
    config: &PrintConfig,
) -> RgbImage {
    let (width, height) = dithered_image.dimensions();
    let (min_pixels_x, min_pixels_y) = constraints.calculate_min_pixels(config, width, height);
    
    let mut result = dithered_image.clone();
    
    if constraints.erosion_dilation_passes > 0 {
        result = apply_morphological_operations(&result, palette, constraints.erosion_dilation_passes);
    }
    
    if constraints.merge_small_features {
        result = merge_small_features(&result, palette, min_pixels_x, min_pixels_y);
    }
    
    result
}

fn apply_morphological_operations(image: &RgbImage, palette: &ColorPalette, passes: u32) -> RgbImage {
    let mut result = image.clone();
    for _ in 0..passes {
        result = morphological_erosion(&result, palette);
        result = morphological_dilation(&result, palette);
    }
    result
}

fn morphological_erosion(image: &RgbImage, palette: &ColorPalette) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut result = image.clone();
    
    for y in 1..height-1 {
        for x in 1..width-1 {
            let center_layer = palette.get_layer_count_for_color(image.get_pixel(x, y));
            let mut min_layer = center_layer;
            
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    let nx = (x as i32 + dx) as u32;
                    let ny = (y as i32 + dy) as u32;
                    let neighbor_layer = palette.get_layer_count_for_color(image.get_pixel(nx, ny));
                    min_layer = min_layer.min(neighbor_layer);
                }
            }
            
            if let Some(color) = palette.get_color_for_layer_count(min_layer) {
                result.put_pixel(x, y, color);
            }
        }
    }
    
    result
}

fn morphological_dilation(image: &RgbImage, palette: &ColorPalette) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut result = image.clone();
    
    for y in 1..height-1 {
        for x in 1..width-1 {
            let center_layer = palette.get_layer_count_for_color(image.get_pixel(x, y));
            let mut max_layer = center_layer;
            
            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    let nx = (x as i32 + dx) as u32;
                    let ny = (y as i32 + dy) as u32;
                    let neighbor_layer = palette.get_layer_count_for_color(image.get_pixel(nx, ny));
                    max_layer = max_layer.max(neighbor_layer);
                }
            }
            
            if let Some(color) = palette.get_color_for_layer_count(max_layer) {
                result.put_pixel(x, y, color);
            }
        }
    }
    
    result
}

fn merge_small_features(image: &RgbImage, palette: &ColorPalette, min_width: u32, min_height: u32) -> RgbImage {
    let (width, height) = image.dimensions();
    let mut result = image.clone();
    let mut visited = vec![vec![false; width as usize]; height as usize];
    
    for y in 0..height {
        for x in 0..width {
            if !visited[y as usize][x as usize] {
                let pixel = *image.get_pixel(x, y);
                let (feature_pixels, bounds) = flood_fill_get_feature(image, x, y, &pixel, &mut visited);
                
                let feature_width = bounds.2 - bounds.0 + 1;
                let feature_height = bounds.3 - bounds.1 + 1;
                
                if feature_width < min_width || feature_height < min_height {
                    let replacement_color = get_surrounding_average_color(image, &feature_pixels, palette);
                    for (fx, fy) in feature_pixels {
                        result.put_pixel(fx, fy, replacement_color);
                    }
                }
            }
        }
    }
    
    result
}

fn flood_fill_get_feature(
    image: &RgbImage,
    start_x: u32, start_y: u32,
    target_color: &Rgb<u8>,
    visited: &mut Vec<Vec<bool>>,
) -> (Vec<(u32, u32)>, (u32, u32, u32, u32)) {
    let (width, height) = image.dimensions();
    let mut pixels = Vec::new();
    let mut stack = vec![(start_x, start_y)];
    let mut bounds = (start_x, start_y, start_x, start_y);
    
    while let Some((x, y)) = stack.pop() {
        if x >= width || y >= height || visited[y as usize][x as usize] || 
           *image.get_pixel(x, y) != *target_color {
            continue;
        }
        
        visited[y as usize][x as usize] = true;
        pixels.push((x, y));
        
        bounds.0 = bounds.0.min(x);
        bounds.1 = bounds.1.min(y);
        bounds.2 = bounds.2.max(x);
        bounds.3 = bounds.3.max(y);
        
        if x > 0 { stack.push((x - 1, y)); }
        if x < width - 1 { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y < height - 1 { stack.push((x, y + 1)); }
    }
    
    (pixels, bounds)
}

fn get_surrounding_average_color(image: &RgbImage, feature_pixels: &[(u32, u32)], palette: &ColorPalette) -> Rgb<u8> {
    let mut color_counts = HashMap::new();
    let (width, height) = image.dimensions();
    
    for &(x, y) in feature_pixels {
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 { continue; }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let neighbor_pixel = *image.get_pixel(nx as u32, ny as u32);
                    if !feature_pixels.contains(&(nx as u32, ny as u32)) {
                        *color_counts.entry(neighbor_pixel).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    
    color_counts.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(color, _)| color)
        .unwrap_or(palette.colors[0])
}