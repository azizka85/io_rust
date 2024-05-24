use std::{f64::consts, fs, io};

fn main() {
    let dir = "data/time series/1d";

    let res = fs::create_dir_all(dir);

    println!("Create dir: {:?}", res);

    let path = format!("{dir}/data.csv");

    let lx = 1.0;

    let nx = 11;

    let dx = lx / (nx - 1) as f64;

    let mut temperature = vec![0.; nx];
    let mut pressure = vec![0.; nx];
    let mut humidity = vec![0.; nx];

    for m in 0..10 {
        for i in 0..nx {
            let pi = consts::PI;

            let x = dx * i as f64;

            let t = 2. * pi * m as f64 / 10.;
            let s = 2. * pi * x;

            temperature[i] = 20. * f64::sin(s + t);
            pressure[i] = 1. + f64::cos(s + t) / 10.;
            humidity[i] = 1. - f64::exp(x + m as f64 / 10.)/f64::exp(3.);
        }

        let res = write_file(&temperature, &pressure, &humidity, dx, nx, &path, m);

        println!("Writing {m}: {:?}", res);
    }
}

fn write_file(
    temperature: &Vec<f64>, pressure: &Vec<f64>, humidity: &Vec<f64>, 
    dx: f64, nx: usize,
    path: &str, num: usize
) -> io::Result<()> {
    let mut writer = csv::Writer::from_path(format!("{path}.{num}"))?;

    writer.write_record(&["x", "humidity", "pressure", "temperature"])?;

    for i in 0..nx {
        let x = dx * i as f64;

        writer.write_record(&[
            format!("{:.3}", x),
            format!("{:.3}", humidity[i]),
            format!("{:.3}", pressure[i]),
            format!("{:.3}", temperature[i])
        ])?;
    }

    Ok(())
}
