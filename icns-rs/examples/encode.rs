use icns_rs::{IcnsEncoder, IconFormats};
use image::open;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Open the image
    let image = match open("example.png") {
        Ok(image) => image,
        Err(e) => {
            println!("Error opening file: {}", e);
            return Ok(());
        }
    };

    // Create the encoder
    let mut encoder = IcnsEncoder::new();

    encoder.data(image);
    encoder.formats(IconFormats::recommended());

    // Encode the image
    let data = match encoder.build() {
        Ok(data) => data,
        Err(e) => {
            println!("Error encoding image: {}", e);
            return Ok(());
        }
    };

    // Write data to file
    let mut file = File::create("example.icns")?;
    file.write_all(&data)?;

    Ok(())
}
