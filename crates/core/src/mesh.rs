use threemf::{model::{Triangle, Triangles, Vertex, Vertices}, Mesh};

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}


pub fn generate_box(
    vertices: &mut Vertices,
    triangles: &mut Triangles,
    x: f32, y: f32, z: f32,
    width: f32, height: f32, depth: f32,
) {
    let base_index = vertices.vertex.len();
    
    // 8 vertices of the box
    let positions = [
        (x, y, z), (x + width, y, z), (x + width, y + height, z), (x, y + height, z),
        (x, y, z + depth), (x + width, y, z + depth), (x + width, y + height, z + depth), (x, y + height, z + depth),
    ];
    
    for (px, py, pz) in &positions {
        vertices.vertex.push(Vertex { x: *px as f64, y: *py as f64, z: *pz as f64 });
    }
    
    // 6 faces, 2 triangles each
    let faces = [
        (0, 3, 2, 1), // bottom
        (4, 5, 6, 7), // top
        (0, 1, 5, 4), // front
        (1, 2, 6, 5), // right
        (2, 3, 7, 6), // back
        (3, 0, 4, 7), // left
    ];
    
    for (v0, v1, v2, v3) in &faces {
        add_quad_triangles(triangles, base_index, *v0, *v1, *v2, *v3);
    }
}

fn add_quad_triangles(triangles: &mut Triangles, base: usize, v0: usize, v1: usize, v2: usize, v3: usize) {
    triangles.triangle.push(Triangle { v1: base + v0, v2: base + v1, v3: base + v2 });
    triangles.triangle.push(Triangle { v1: base + v0, v2: base + v2, v3: base + v3 });
}

pub fn calculate_normal(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> Vertex {
    let edge1 = Vertex { x: v1.x - v0.x, y: v1.y - v0.y, z: v1.z - v0.z };
    let edge2 = Vertex { x: v2.x - v0.x, y: v2.y - v0.y, z: v2.z - v0.z };
    
    let normal = Vertex {
        x: edge1.y * edge2.z - edge1.z * edge2.y,
        y: edge1.z * edge2.x - edge1.x * edge2.z,
        z: edge1.x * edge2.y - edge1.y * edge2.x,
    };
    
    let length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    if length > 0.0 {
        Vertex { x: normal.x / length, y: normal.y / length, z: normal.z / length }
    } else {
        Vertex { x: 0.0, y: 1.0, z: 0.0 }
    }
}

// pub fn export_to_stl(mesh: &Mesh, filename: &str) -> std::io::Result<()> {
//     use std::fs::File;
//     use std::io::Write;
    
//     let mut file = File::create(filename)?;
//     writeln!(file, "solid object")?;
    
//     for triangle in &mesh.triangles.triangle {
//         let v0 = &mesh.vertices.vertex[triangle.v1];
//         let v1 = &mesh.vertices.vertex[triangle.v2];
//         let v2 = &mesh.vertices.vertex[triangle.v3];
//         let normal = calculate_normal(v0, v1, v2);
        
//         writeln!(file, "  facet normal {} {} {}", normal.x, normal.y, normal.z)?;
//         writeln!(file, "    outer loop")?;
//         writeln!(file, "      vertex {} {} {}", v0.x, v0.y, v0.z)?;
//         writeln!(file, "      vertex {} {} {}", v1.x, v1.y, v1.z)?;
//         writeln!(file, "      vertex {} {} {}", v2.x, v2.y, v2.z)?;
//         writeln!(file, "    endloop")?;
//         writeln!(file, "  endfacet")?;
//     }
    
//     writeln!(file, "endsolid object")?;
//     Ok(())
// }
