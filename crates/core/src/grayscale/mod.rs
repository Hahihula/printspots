use std::{fs::File, io::BufWriter};

use threemf::model::{Base, BaseMaterials, Item, Model, Object};

use crate::utils::PrintObjects;

pub mod calibration;

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