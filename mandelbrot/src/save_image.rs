extern crate image;

use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
pub fn pgm_p5(width: u32, height: u32, data: &[u8], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.pgm", filename))?;
    // PGM headers
    writeln!(file, "P5")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;
    // write image data
    file.write_all(data)?;
    Ok(())
}

#[allow(dead_code)]
pub fn pgm_p6(width: u32, height: u32, data: &[u8], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(format!("{}.pgm", filename))?;
    // PGM headers
    writeln!(file, "P6")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;
    // write image data
    let data = data.into_iter().fold(vec![], |mut acc, cur| {
        acc.extend(palette(*cur));
        acc
    });
    file.write_all(&data)?;
    Ok(())
}

#[allow(dead_code)]
pub fn png(width: u32, height: u32, data: &[u8], filename: &str) -> std::io::Result<()> {
    let buf = data.into_iter().fold(vec![], |mut acc, cur| {
        acc.extend(palette(*cur));
        acc
    });
    image::save_buffer_with_format(
        format!("{}.png", filename),
        &buf,
        width,
        height,
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )
    .unwrap();
    Ok(())
}

fn palette(i: u8) -> Vec<u8> {
    match i {
        0 => vec![255, 255, 255],
        _ => vec![(3 * i) % 255, (5 * i) % 255, (7 * i) % 255],
    }
}
