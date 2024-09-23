use crate::complex::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

pub fn mandelbort(
    x_range: (f64, f64),
    y_range: (f64, f64),
    resolution: f64,
) -> (u32, u32, Vec<u8>) {
    // image dimensions
    let width = ((x_range.1 - x_range.0) / resolution).floor() as u32;
    let height = ((y_range.1 - y_range.0) / resolution).floor() as u32;

    // num of threads
    let num_cpu = std::thread::available_parallelism().unwrap();

    // share state
    let results: Arc<Mutex<BTreeMap<u32, Vec<u8>>>> = Arc::new(Mutex::new(BTreeMap::new()));

    println!("Iniciamos el cálculo con {} hilos.", num_cpu);
    let start_time = Instant::now();
    let mut handles = vec![];
    for row in 0..height {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let data_row: Vec<u8> = (0..width)
                .map(|col| {
                    let re = x_range.0 + resolution * col as f64;
                    let im = y_range.1 - resolution * row as f64;
                    color_coord(Complex::new(re, im))
                })
                .collect();
            results.lock().unwrap().insert(row, data_row);
        });
        handles.push(handle);
    }

    // join results
    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    let image = results.values().fold(Vec::new(), |mut acc, row| {
        acc.extend(row);
        acc
    });
    println!("Fin del cálculo en {:?}", start_time.elapsed());
    (width, height, image)
}

fn color_coord(c: Complex<f64>) -> u8 {
    let mut count: u8 = 0;
    let mut z: Complex<f64> = Complex::new(0f64, 0f64);
    while z.norm_sqrt_f64() < 4.0 && count < u8::MAX {
        z = z * z + c;
        count += 1;
    }
    count
}
