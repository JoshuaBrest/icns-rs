pub mod icns_format;
pub mod image_encoder;
pub mod image_types;
pub mod packbits;

use icns_format::IconFamily;
use image::DynamicImage;
use image_encoder::ImageBuilder;
pub use image_types::IconFormats;

/// The main encoder struct
/// Create a new encoder with `IcnsEncoder::new()`
pub struct IcnsEncoder {
    data: DynamicImage,
    formats: Vec<IconFormats>,
}

impl IcnsEncoder {
    /// Creates a new IcnsEncoder
    ///
    /// Usage:
    /// ```no_run
    /// use icns_rs::{IcnsEncoder, IconFormats};
    /// use image::open;
    /// use std::fs::File;
    /// use std::io::prelude::*;
    ///
    /// // Open the image
    /// let image = match open("512x512@2.png") {
    ///     Ok(image) => image,
    ///     Err(e) => {
    ///         println!("Error: {}", e);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// // Create the encoder
    /// let mut encoder = IcnsEncoder::new();
    ///
    /// encoder.data(image);
    /// encoder.formats(IconFormats::recommended());
    ///
    /// // Encode the image
    /// let data = match encoder.build() {
    ///     Ok(data) => data,
    ///     Err(e) => {
    ///         println!("Error ould not encode image");
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// // Write data to file
    /// let mut file = match File::create("example.icns") {
    ///     Ok(file) => file,
    ///     Err(e) => {
    ///         println!("Error: {}", e);
    ///         std::process::exit(1);
    ///     }
    /// };
    ///
    /// match file.write_all(&data) {
    ///     Ok(_) => println!("Successfully wrote to file"),
    ///     Err(e) => {
    ///         println!("Error: {}", e);
    ///         std::process::exit(1);
    ///     }
    /// };
    /// ```
    pub fn new() -> Self {
        Self {
            data: DynamicImage::new_rgb8(1, 1),
            formats: Vec::new(),
        }
    }

    /// Sets the image data. Encode a png and pass it as a DynamicImage.
    pub fn data(&mut self, data: DynamicImage) -> &mut Self {
        self.data = data;

        self
    }

    /// Sets the image formats to be encoded
    pub fn formats(&mut self, formats: Vec<IconFormats>) -> &mut Self {
        self.formats = formats;

        self
    }

    /// Encodes the image as an ICNS file
    pub fn build(&self) -> Result<Box<[u8]>, String> {
        let mut file = IconFamily::new();

        let mut image_encoder = ImageBuilder::new();
        image_encoder.data(self.data.clone());

        for format in &self.formats {
            let image = image_encoder.format(format.clone()).build()?;

            file.add_data(image);
        }

        Ok(file.build())
    }
}
