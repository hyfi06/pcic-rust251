mod complex;
use std::fs::File;
use std::io::Write;
use std::{
    sync::{Arc, Mutex},
    thread, vec,
};

use complex::*;

const HIGHT: i32 = 10000;
const WIDTH: i32 = 10000;
const X_MAX: f64 = 2.0;
const X_MIN: f64 = -2.0;
const Y_MAX: f64 = 2.0;
const Y_MIN: f64 = -2.0;

fn main() {
    let num_cpu = std::thread::available_parallelism().unwrap();
    let results: Arc<Mutex<Vec<(i32, Vec<u8>)>>> = Arc::new(Mutex::new(Vec::new()));
    let rows = Arc::new(Mutex::new((0..HIGHT).into_iter()));
    
    println!("Iniciamos el cálculo con {} hilos.", num_cpu);
    let mut handles = vec![];
    for _ in 0..num_cpu.into() {
        let results = Arc::clone(&results);
        let rows = Arc::clone(&rows);
        let handle = thread::spawn(move || {
            while let Some(row) = rows.lock().unwrap().next() {
                let data_row: Vec<u8> = (0..WIDTH)
                    .map(|col| color_coord(pixel_to_coord(row, col)))
                    .collect();
                results.lock().unwrap().push((row, data_row));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    results.lock().unwrap().sort();
    let image = results
        .lock()
        .unwrap()
        .iter()
        .fold(Vec::new(), |mut acc: Vec<u8>, (_, row)| {
            acc.extend(row);
            acc
        });
    save_image(
        WIDTH,
        HIGHT,
        &image,
        &format!("mandelbrot_{WIDTH}_{HIGHT}.pgm"),
    )
    .unwrap();
}

fn pixel_to_coord(row: i32, col: i32) -> Complex<f64> {
    let dx: f64 = (X_MAX - X_MIN) / (f64::from(WIDTH) - 1.0);
    let dy: f64 = (Y_MAX - Y_MIN) / (f64::from(HIGHT) - 1.0);
    let re = X_MIN + dx * f64::from(col);
    let im = Y_MAX - dy * f64::from(row);
    Complex::new(re, im)
}

fn color_coord(c: Complex<f64>) -> u8 {
    let mut count: u8 = 0;
    let mut z: Complex<f64> = Complex::new(0f64, 0f64);
    while z.norm64() < 4.0 && count < u8::MAX {
        z = z * z + c;
        count += 1;
    }
    count
}

fn save_image(width: i32, height: i32, data: &[u8], filename: &str) -> std::io::Result<()> {
    // Abre el archivo para escribir
    let mut file = File::create(filename)?;

    // Escribe el encabezado del archivo PGM
    writeln!(file, "P5")?; // El formato "P5" indica una imagen binaria en escala de grises
    writeln!(file, "{} {}", width, height)?; // Ancho y alto de la imagen
    writeln!(file, "255")?; // El valor máximo de intensidad de gris (0-255)

    // Escribe los datos de la imagen
    file.write_all(data)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn limits() {
        let ul = pixel_to_coord(0, 0);
        let dl = pixel_to_coord(HIGHT - 1, 0);
        let ur = pixel_to_coord(0, WIDTH - 1);
        let dr = pixel_to_coord(HIGHT - 1, WIDTH - 1);
        assert_eq!(Complex::new(X_MIN, Y_MAX),ul);
        assert_eq!(Complex::new(X_MIN, Y_MIN),dl);
        assert_eq!(Complex::new(X_MAX, Y_MAX),ur);
        assert_eq!(Complex::new(X_MAX, Y_MIN),dr);
    }
}
