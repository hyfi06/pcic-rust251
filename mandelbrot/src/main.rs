mod complex;
mod config;
mod mandelbort;
mod save_image;

use mandelbort::mandelbort;
fn main() {
    let filename = "mandelbort";
    let (width, height, data) = mandelbort(
        (config::DEFAULT_X_MIN, config::DEFAULT_X_MAX),
        (config::DEFAULT_Y_MIN, config::DEFAULT_Y_MAX),
        config::DEFAULT_RESOLUTION,
    );
    save_image::png(width, height, &data, filename).unwrap();
}
