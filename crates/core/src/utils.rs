use threemf::model::{Vertices, Triangles};

pub struct Mesh {
    pub vertices: Vertices,
    pub triangles: Triangles,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vertices { vertex: Vec::new() },
            triangles: Triangles { triangle: Vec::new() },
        }
    }
}

pub struct PrintObjects {
    pub black_mesh: Mesh,
    pub white_mesh: Mesh,
}