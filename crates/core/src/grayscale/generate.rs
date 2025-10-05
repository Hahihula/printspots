use std::collections::HashMap;

use image::RgbImage;
use threemf::model::Triangle;

use crate::{config::PrintConfig, grayscale::ColorPalette, mesh::{generate_box, Rectangle}, utils::{Mesh, PrintObjects}};

pub fn generate_image(
    image: &RgbImage,
    palette: &ColorPalette,
    config: &PrintConfig,
    flat_top: bool,
) -> PrintObjects {
    if flat_top {
        // Use existing vectorized approach for flat top
        generate_image_objects_vectorized(image, palette, config, flat_top)
    } else {
        // For variable height, create optimized layer structure
        generate_variable_height_optimized(image, palette, config)
    }
}

pub fn generate_image_objects_vectorized(
    image: &RgbImage,
    palette: &ColorPalette,
    config: &PrintConfig,
    flat_top: bool,
) -> PrintObjects {
    let regions = vectorize_image_to_regions(image, palette);
    let (width, height) = image.dimensions();
    let (pixel_width, pixel_height) = config.pixel_size(width, height);
    
    let mut black_mesh = Mesh::new();
    let mut white_mesh = Mesh::new();
    
    for (&layer_count, rectangles) in &regions {
        for rect in rectangles {
            let world_x = rect.x as f32 * pixel_width;
            let world_y = rect.y as f32 * pixel_height;
            let rect_width = rect.width as f32 * pixel_width;
            let rect_height = rect.height as f32 * pixel_height;
            
            let black_height = config.calculate_black_thickness(layer_count, flat_top);
            
            generate_box(
                &mut black_mesh.vertices,
                &mut black_mesh.triangles,
                world_x, world_y, 0.0,
                rect_width, rect_height, black_height,
            );
            
            if layer_count > 0 {
                let white_height = layer_count as f32 * config.layer_thickness;
                generate_box(
                    &mut white_mesh.vertices,
                    &mut white_mesh.triangles,
                    world_x, world_y, black_height,
                    rect_width, rect_height, white_height,
                );
            }
        }
    }
    
    PrintObjects { black_mesh, white_mesh }
}

/// Generate ultra-optimized variable height objects
fn generate_variable_height_optimized(
    image: &RgbImage,
    palette: &ColorPalette,
    config: &PrintConfig,
) -> PrintObjects {
    let (width, height) = image.dimensions();
    let (pixel_width, pixel_height) = config.pixel_size(width, height);
    
    // Calculate total image bounds
    let total_width = width as f32 * pixel_width;
    let total_height = height as f32 * pixel_height;
    
    // Create single black base covering entire image
    let mut black_mesh = Mesh::new();
    generate_box(
        &mut black_mesh.vertices,
        &mut black_mesh.triangles,
        0.0, 0.0, 0.0,
        total_width, total_height, config.base_thickness,
    );
    
    // Group white regions by layer count and vectorize each group
    let layer_map = image_to_layer_map(image, palette);
    let mut white_mesh = Mesh::new();
    
    // Group pixels by their layer count
    let mut layer_groups: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    for ((x, y), &layer_count) in &layer_map {
        if layer_count > 0 {
            layer_groups.entry(layer_count).or_default().push((*x, *y));
        }
    }
    
    // Create vectorized regions for each layer count
    for (&layer_count, pixels) in &layer_groups {
        let rectangles = pixels_to_rectangles(pixels);
        let z_offset = config.base_thickness;
        let layer_height = layer_count as f32 * config.layer_thickness;
        
        for rect in rectangles {
            let world_x = rect.x as f32 * pixel_width;
            let world_y = rect.y as f32 * pixel_height;
            let rect_width = rect.width as f32 * pixel_width;
            let rect_height = rect.height as f32 * pixel_height;
            
            generate_box(
                &mut white_mesh.vertices,
                &mut white_mesh.triangles,
                world_x, world_y, z_offset,
                rect_width, rect_height, layer_height,
            );
        }
    }
    
    PrintObjects { black_mesh, white_mesh }
}

fn vectorize_image_to_regions(image: &RgbImage, palette: &ColorPalette) -> HashMap<u32, Vec<Rectangle>> {
    let layer_map = image_to_layer_map(image, palette);
    let mut regions = HashMap::new();
    let mut layer_pixels: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    
    for ((x, y), &layer_count) in &layer_map {
        layer_pixels.entry(layer_count).or_default().push((*x, *y));
    }
    
    for (&layer_count, pixels) in &layer_pixels {
        let rectangles = pixels_to_rectangles(pixels);
        regions.insert(layer_count, rectangles);
    }
    
    regions
}

fn image_to_layer_map(image: &RgbImage, palette: &ColorPalette) -> HashMap<(u32, u32), u32> {
    let (width, height) = image.dimensions();
    let mut layer_map = HashMap::with_capacity((width * height) as usize);
    
    for y in 0..height {
        for x in 0..width {
            let pixel = *image.get_pixel(x, y);
            let layer_count = palette.get_layer_count_for_color(&pixel);
            layer_map.insert((x, y), layer_count);
        }
    }
    
    layer_map
}

// Helper function from existing code
fn pixels_to_rectangles(pixels: &[(u32, u32)]) -> Vec<Rectangle> {
    let mut rectangles = Vec::new();
    let mut pixel_set: std::collections::HashSet<(u32, u32)> = pixels.iter().cloned().collect();
    
    while let Some(&(start_x, start_y)) = pixel_set.iter().next() {
        let rect = find_largest_rectangle(&pixel_set, start_x, start_y);
        
        for y in rect.y..rect.y + rect.height {
            for x in rect.x..rect.x + rect.width {
                pixel_set.remove(&(x, y));
            }
        }
        
        rectangles.push(rect);
    }
    
    rectangles
}

/// Optimize meshes by consolidating layers of the same height
pub fn optimize_mesh_layers(objects: PrintObjects, flat_top: bool) -> PrintObjects {
    if flat_top {
        // For flat top, each color might have different heights per region, so use existing vectorization
        objects // TODO: Could still be optimized but more complex
    } else {
        // For variable height, we can create large consolidated layers
        optimize_variable_height_mesh(&objects)
    }
}

/// Optimize variable height meshes by creating consolidated layers
fn optimize_variable_height_mesh(objects: &PrintObjects) -> PrintObjects {
    PrintObjects {
        black_mesh: consolidate_uniform_layer(&objects.black_mesh),
        white_mesh: consolidate_by_height(&objects.white_mesh),
    }
}

/// Consolidate mesh where all geometry is at the same height (like black base)
fn consolidate_uniform_layer(mesh: &Mesh) -> Mesh {
    if mesh.vertices.vertex.is_empty() {
        return Mesh::new();
    }
    
    // Find the Z-level and bounds of all geometry
    let mut min_x = f64::MAX;
    let mut max_x = f64::MIN;
    let mut min_y = f64::MAX;
    let mut max_y = f64::MIN;
    let mut z_bottom = f64::MAX;
    let mut z_top = f64::MIN;
    
    for vertex in &mesh.vertices.vertex {
        min_x = min_x.min(vertex.x);
        max_x = max_x.max(vertex.x);
        min_y = min_y.min(vertex.y);
        max_y = max_y.max(vertex.y);
        z_bottom = z_bottom.min(vertex.z);
        z_top = z_top.max(vertex.z);
    }
    
    // Create a single large box covering the entire area
    let mut optimized_mesh = Mesh::new();
    let thickness = z_top - z_bottom;
    
    if thickness > 0.0 {
        generate_box(
            &mut optimized_mesh.vertices,
            &mut optimized_mesh.triangles,
            min_x as f32, min_y as f32, z_bottom as f32,
            (max_x - min_x) as f32, (max_y - min_y) as f32, thickness as f32,
        );
    }
    
    optimized_mesh
}

/// Consolidate mesh by grouping regions of the same height
fn consolidate_by_height(mesh: &Mesh) -> Mesh {
    if mesh.vertices.vertex.is_empty() {
        return Mesh::new();
    }
    
    // Group triangles by their Z-levels (height)
    let mut height_groups: HashMap<(i32, i32), Vec<&Triangle>> = HashMap::new(); // (z_bottom, z_top) -> triangles
    
    for triangle in &mesh.triangles.triangle {
        let v0 = &mesh.vertices.vertex[triangle.v1];
        let v1 = &mesh.vertices.vertex[triangle.v2];
        let v2 = &mesh.vertices.vertex[triangle.v3];
        
        // Find the Z-range of this triangle
        let min_z = v0.z.min(v1.z).min(v2.z);
        let max_z = v0.z.max(v1.z).max(v2.z);
        
        // Quantize Z values to group similar heights (0.01mm precision)
        let z_bottom_key = (min_z * 100.0).round() as i32;
        let z_top_key = (max_z * 100.0).round() as i32;
        
        height_groups.entry((z_bottom_key, z_top_key)).or_default().push(triangle);
    }
    
    // For each height group, find the bounding rectangle and create one box
    let mut optimized_mesh = Mesh::new();
    
    for ((z_bottom_key, z_top_key), triangles) in height_groups {
        let z_bottom = z_bottom_key as f64 / 100.0;
        let z_top = z_top_key as f64 / 100.0;
        let thickness = z_top - z_bottom;
        
        if thickness <= 0.001 { // Skip very thin layers (likely artifacts)
            continue;
        }
        
        // Find bounding rectangle for this height group
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        
        for triangle in &triangles {
            for &vertex_idx in [triangle.v1, triangle.v2, triangle.v3].iter() {
                let vertex = &mesh.vertices.vertex[vertex_idx];
                min_x = min_x.min(vertex.x);
                max_x = max_x.max(vertex.x);
                min_y = min_y.min(vertex.y);
                max_y = max_y.max(vertex.y);
            }
        }
        
        // Create one box for this height level
        generate_box(
            &mut optimized_mesh.vertices,
            &mut optimized_mesh.triangles,
            min_x as f32, min_y as f32, z_bottom as f32,
            (max_x - min_x) as f32, (max_y - min_y) as f32, thickness as f32,
        );
    }
    
    optimized_mesh
}

fn find_largest_rectangle(pixel_set: &std::collections::HashSet<(u32, u32)>, start_x: u32, start_y: u32) -> Rectangle {
    let mut width = 1;
    let mut height = 1;
    
    // Expand right
    while pixel_set.contains(&(start_x + width, start_y)) {
        let mut column_available = true;
        for y in start_y..start_y + height {
            if !pixel_set.contains(&(start_x + width, y)) {
                column_available = false;
                break;
            }
        }
        if column_available {
            width += 1;
        } else {
            break;
        }
    }
    
    // Expand down
    while {
        let mut row_available = true;
        for x in start_x..start_x + width {
            if !pixel_set.contains(&(x, start_y + height)) {
                row_available = false;
                break;
            }
        }
        row_available
    } {
        height += 1;
    }
    
    Rectangle { x: start_x, y: start_y, width, height }
}