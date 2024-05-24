use std::{f64::consts, fs, io};

fn main() {
    let dir = "data/time series/structured grid";

    let res = fs::create_dir_all(dir);

    println!("Create dir: {:?}", res);

    let path = format!("{dir}/temperature.csv");

    let nx = 11;
    let ny = 11;
    let nz = 11;

    let lx = 1.;
    let ly = 1.;
    let lz = 1.;

    let dx = lx / (nx - 1) as f64;
    let dy = ly / (ny - 1) as f64;
    let dz = lz / (nz - 1) as f64;

    let mut temperature = vec![vec![vec![0.; nz]; ny]; nx];

    for m in 0..10 {
        for k in 0..nz {
            for j in 0..ny {
                for i in 0..nx {
                    let x = dx * i as f64;
                    let y = dy * j as f64;
                    let z = dz * k as f64;

                    let pi = consts::PI;

                    let t = 2. * pi * m as f64 / 10.;
                    let s = 2. * pi * (x + y + z)/3.;

                    temperature[i][j][k] = f64::sin(s + t);
                }
            }
        }

        let res = write_file(&temperature, dx, dy, dz, nx, ny, nz, &path, m);

        println!("Writing {m}: {:?}", res);
    }     
}

fn write_file(
    temperature: &Vec<Vec<Vec<f64>>>, 
    dx: f64, dy: f64, dz: f64,
    nx: usize, ny: usize, nz: usize,
    path: &str, num: usize
) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(format!("{path}.{num}"))?;

    writer.write_record(&["x", "y", "z", "temperature"])?;

    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                let x = dx * i as f64;
                let y = dy * j as f64;
                let z = dz * k as f64;

                writer.write_record(&[
                    format!("{:.3}", x),
                    format!("{:.3}", y),
                    format!("{:.3}", z),
                    format!("{:.3}", temperature[i][j][k])
                ])?;
            }
        }
    }

    Ok(())
}
