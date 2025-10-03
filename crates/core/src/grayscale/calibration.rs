use crate::config::PrintConfig;
use crate::utils::{PrintObjects, Mesh};
use crate::mesh::generate_box;

pub fn generate_calibration_objects(config: &PrintConfig, square_size: f32, flat_top: bool) -> PrintObjects {
    let mut black_mesh = Mesh::new();
    let mut white_mesh = Mesh::new();
    
    let grid_size = ((config.max_layers + 1.0) as f32).sqrt().ceil() as u32;
    
    for layer_count in 0..=config.max_layers as u32 {
        let grid_x = layer_count % grid_size;
        let grid_y = layer_count / grid_size;
        
        if grid_y >= grid_size {
            break;
        }
        
        let x_offset = grid_x as f32 * square_size;
        let y_offset = grid_y as f32 * square_size;
        let black_thickness = config.calculate_black_thickness(layer_count, flat_top);
        let total_height = config.calculate_total_height(layer_count, flat_top);

        // Generate black base
        generate_box(
            &mut black_mesh.vertices,
            &mut black_mesh.triangles,
            x_offset, y_offset, 0.0,
            square_size, square_size, black_thickness,
        );
        
        // Generate white layer if needed
        if total_height > black_thickness {
            generate_box(
                &mut white_mesh.vertices,
                &mut white_mesh.triangles,
                x_offset, y_offset, black_thickness,
                square_size, square_size, total_height - black_thickness,
            );
        }
    }
    
    PrintObjects { black_mesh, white_mesh }
}