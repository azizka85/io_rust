use std::{fs, io, collections, f64::consts};

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

    let dir = "data";

    let res = fs::create_dir_all(dir);

    println!("Create dir: {:?}", res);

    let path = format!("{dir}/temperature.vtk");

    let temperature = (0..mesh.total_node_count())
        .map(|i| 20. * f64::sin(2. * consts::PI * i as f64 / mesh.total_node_count() as f64))
        .collect();

    let res = write_file(&mesh, &temperature, &path);

    println!("Writing: {:?}", res);

    Ok(())
}

fn read_mesh(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}

fn write_file(
    mesh: &mshio::MshFile<u64, i32, f64>,
    temperature: &Vec<f64>, path: &str
) -> io::Result<()> {
    let element_type_map = collections::HashMap::from([
        (mshio::ElementType::Qua4, 9)
    ]);

    let mut file = fs::File::create(path)?;

    io::Write::write_all(&mut file, "# vtk DataFile Version 2.0\n".as_bytes())?;
    io::Write::write_all(&mut file, "Temperature distribution\n".as_bytes())?;
    io::Write::write_all(&mut file, "ASCII\n".as_bytes())?;
    io::Write::write_all(&mut file, "DATASET UNSTRUCTURED_GRID\n\n".as_bytes())?;

    io::Write::write_all(
        &mut file, 
        format!("POINTS {} float\n", mesh.total_node_count()).as_bytes()
    )?;

    if let Some(nodes) = mesh.data.nodes.as_ref() {
        for block in &nodes.node_blocks {
            for node in &block.nodes {
                io::Write::write_all(
                    &mut file, 
                    format!("{:.3} {:.3} {:.3}\n", node.x, node.y, node.z).as_bytes()
                )?;
            }
        }
    }

    io::Write::write_all(&mut file, "\n".as_bytes())?;

    let mut size = 0;

    if let Some(elems) = mesh.data.elements.as_ref() {
        for block in &elems.element_blocks {
            for elem in &block.elements {
                size += elem.nodes.len() + 1;
            }
        }
    }

    io::Write::write_all(
        &mut file, 
        format!("CELLS {} {size}\n", mesh.total_element_count()).as_bytes()
    )?;

    if let Some(elems) = mesh.data.elements.as_ref() {
        for block in &elems.element_blocks {
            for elem in &block.elements {
                let nodes_str = elem.nodes.iter()
                    .map(|e| (e - 1).to_string())
                    .collect::<Vec<_>>()
                    .join(" ");

                io::Write::write_all(
                    &mut file, format!("{} {}\n", 
                    elem.nodes.len(), nodes_str).as_bytes()
                )?;
            }
        }
    }

    io::Write::write_all(
        &mut file, 
        format!("\nCELL_TYPES {}\n", mesh.total_element_count()).as_bytes()
    )?;

    if let Some(elems) = mesh.data.elements.as_ref() {
        for block in &elems.element_blocks {
            for _ in &block.elements {
                io::Write::write_all(
                    &mut file, format!("{}\n", element_type_map[&block.element_type]).as_bytes()
                )?;
            }
        }
    }

    io::Write::write_all(
        &mut file, 
        format!("\nPOINT_DATA {}\n", mesh.total_node_count()).as_bytes()
    )?;

    io::Write::write_all(&mut file, "SCALARS scalars float 1\n".as_bytes())?;
    io::Write::write_all(&mut file, "LOOKUP_TABLE default\n".as_bytes())?;

    for i in 0..mesh.total_node_count() {
        io::Write::write_all(&mut file, format!("{:.3}\n", temperature[i]).as_bytes())?;
    }    

    Ok(())
}
