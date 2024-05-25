use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let path = "mesh_data/circle_2d.msh";

    let data = read_mesh(path)?;

    let mesh = mshio::parse_msh_bytes(&data).unwrap();    

    println!("Header: {:#?}\n", mesh.header);

    println!("Element types: {:#?}\n", mesh.count_element_types());
    println!("Curves: {}", mesh.curve_count());
    println!("Points: {}", mesh.point_count());
    println!("Surfaces: {}", mesh.surface_count());
    println!("Elements: {}", mesh.total_element_count());
    println!("Nodes: {}", mesh.total_node_count());
    println!("Volumes: {}", mesh.volume_count());    

    println!("Elements: {:#?}\n", mesh.data.elements);
    println!("Entities: {:#?}\n", mesh.data.entities);
    println!("Nodes: {:#?}\n", mesh.data.nodes);

    Ok(())
}

fn read_mesh(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}
