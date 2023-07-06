use std::io::Write;

use crate::{icns_format::IcnsDataEntry, image_types::IconFormats, packbits};

use image::{codecs::png::PngEncoder, imageops::FilterType, DynamicImage, ImageEncoder};

/// The ImageBuilder struct
/// This struct is used to build the image data, specifically,
/// resizing the image and encoding it as a RGB, ARGB, mask,
/// or PNG image
pub struct ImageBuilder {
    pub format: IconFormats,
    pub data: DynamicImage,
    pub filter: FilterType,
}

impl ImageBuilder {
    pub fn new() -> Self {
        Self {
            format: IconFormats::IS32,
            data: DynamicImage::new_rgb8(1, 1),
            filter: FilterType::Nearest,
        }
    }

    /// Sets the image format
    /// See the `IconFormats` enum for more information
    pub fn format(&mut self, format: IconFormats) -> &mut Self {
        self.format = format;

        self
    }

    /// Sets the image data. Encode a png and pass it as a DynamicImage.
    pub fn data(&mut self, data: DynamicImage) -> &mut Self {
        self.data = data;

        self
    }

    /// Sets the filter type to be used when resizing the image
    /// - `Nearest`: Nearest neighbor interpolation
    /// - `Triangle`: Triangle interpolation
    /// - `CatmullRom`: Catmull-Rom interpolation
    /// - `Gaussian`: Gaussian interpolation
    ///
    /// The default is `Nearest` because it's the fastest
    pub fn filter(&mut self, filter: FilterType) -> &mut Self {
        self.filter = filter;

        self
    }

    /// Encodes an image as a RGB image
    /// You probably want to use `.build()` instead of this method
    pub fn rgb_image(&self) -> Result<Box<[u8]>, String> {
        let size = self.format.get_size() as u32;
        let resized = self.data.resize(size, size, self.filter);
        let rgb8 = resized.to_rgb8();
        let data = rgb8.pixels().collect::<Vec<_>>();

        let channels = [
            // Offset if the type is it32
            if self.format == IconFormats::IT32 {
                vec![0x00, 0x00, 0x00, 0x00].into_boxed_slice()
            } else {
                Vec::new().into_boxed_slice()
            },
            // Red channel
            packbits::compress(
                data.iter()
                    .map(|pixel| pixel[0])
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            // Green channel
            packbits::compress(
                data.iter()
                    .map(|pixel| pixel[1])
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            // Blue channel
            packbits::compress(
                data.iter()
                    .map(|pixel| pixel[2])
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        ];

        let mut buffer = Vec::with_capacity(channels.iter().map(|c| c.len()).sum::<usize>());

        for b in channels {
            buffer.extend_from_slice(&b);
        }

        Ok(buffer.into_boxed_slice())
    }

    /// Encodes an image as a ARGB
    /// You probably want to use `.build()` instead of this method
    pub fn argb_image(&self) -> Result<Box<[u8]>, String> {
        let size = self.format.get_size() as u32;
        let resized = self.data.resize(size, size, self.filter);
        let rgba8 = resized.to_rgba8();
        let data = rgba8.pixels().collect::<Vec<_>>();

        let channels = [
            // File header
            vec![0x41, 0x52, 0x47, 0x42].into_boxed_slice(), // ARGB
            // Alpha channel
            packbits::compress(
                data.iter()
                    .map(|pixel| pixel[3])
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            // Red channel
            packbits::compress(
                data.iter()
                    .map(|pixel| pixel[0])
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            // Green channel
            packbits::compress(
                data.iter()
                    .map(|pixel| pixel[1])
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            // Blue channel
            packbits::compress(
                data.iter()
                    .map(|pixel| pixel[2])
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
        ];

        let mut buffer = Vec::with_capacity(channels.iter().map(|c| c.len()).sum::<usize>());

        for b in channels {
            buffer.extend_from_slice(&b);
        }

        Ok(buffer.into_boxed_slice())
    }

    /// Encodes an image as a mask
    /// You probably want to use `.build()` instead of this method
    pub fn mask_image(&self) -> Result<Box<[u8]>, String> {
        let size = self.format.get_size() as u32;
        let resized = self.data.resize(size, size, self.filter);
        let luma = resized.to_luma_alpha8();
        let data = luma.pixels().collect::<Vec<_>>();

        // No compression
        let mask = data
            .iter()
            .map(|pixel| pixel[1])
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Ok(mask)
    }

    /// Encodes an image as a PNG
    pub fn png_image(&self) -> Result<Box<[u8]>, String> {
        let size = self.format.get_size() as u32;
        let data = self.data.resize(size, size, self.filter);

        let mut buffer = Vec::new();

        // Required because the PngEncoder drops the writer
        struct WriterProxy<'a> {
            buffer: &'a mut Vec<u8>,
        }

        impl<'a> Write for WriterProxy<'a> {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                self.buffer.write(buf)
            }

            fn flush(&mut self) -> std::io::Result<()> {
                self.buffer.flush()
            }
        }

        let encoder = PngEncoder::new(WriterProxy {
            buffer: &mut buffer,
        });

        let color = data.color();

        let result = encoder.write_image(data.into_bytes().as_slice(), size, size, color);

        match result {
            Ok(_) => Ok(buffer.into_boxed_slice()),
            Err(e) => Err(format!("Failed to encode PNG: {}", e)),
        }
    }

    pub fn build(&self) -> Result<IcnsDataEntry, String> {
        let data = match self.format.get_format() {
            crate::image_types::FileFormat::RGB => self.rgb_image(),
            crate::image_types::FileFormat::ARGB => self.argb_image(),
            crate::image_types::FileFormat::MASK => self.mask_image(),
            crate::image_types::FileFormat::PNG => self.png_image(),
        }?;

        Ok(IcnsDataEntry::new(self.format.get_bytes(), data))
    }
}
