use std::io;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Temperature {
    x: f64,
    y: f64,
    z: f64,
    
    #[serde(rename = "temperature")]
    v: f64
}

fn main() {
    let path = "data/temperature.csv";

    let temperature = [
        Temperature {x: 0.0, y: 0.0, z: 0.0, v: 0.0},
        Temperature {x: 1.0, y: 0.0, z: 0.0, v: 1.0},
        Temperature {x: 0.0, y: 1.0, z: 0.0, v: 2.0},
        Temperature {x: 1.0, y: 1.0, z: 0.0, v: 3.0},
        Temperature {x: -0.5, y: -0.5, z: 1.0, v: 4.0},
        Temperature {x: 0.5, y: -0.5, z: 1.0, v: 5.0},
        Temperature {x: -0.5, y: 0.5, z: 1.0, v: 6.0},
        Temperature {x: 0.5, y: 0.5, z: 1.0, v: 7.0}
    ];

    let res = write_file(&temperature, path);

    println!("Result of writing: {:#?}", res);    

    let res = read_file(path);

    println!(
        "Result of reading: {:#?}", 
        res
    );    
}

fn write_file(temperature: &[Temperature], path: &str) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    for item in temperature {
        writer.serialize(item)?;
    }

    Ok(())
}

fn read_file(path: &str) -> io::Result<Vec<Temperature>> {
    let mut reader = csv::Reader::from_path(path)?;

    let mut temperature = Vec::new();

    for result in reader.deserialize() {
        temperature.push(result?);
    }

    Ok(temperature)
}
