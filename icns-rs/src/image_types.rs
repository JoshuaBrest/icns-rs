#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FileFormat {
    RGB,
    ARGB,
    MASK,
    PNG,
}

/// # ICNS Types
/// These are the types of icons that can be stored in an ICNS file.
/// Not all of them are included, but the most common ones are.
/// The full list can be found at Wikipedia
/// https://en.wikipedia.org/wiki/Apple_Icon_Image_format#Icon_types
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum IconFormats {
    /// - OSName: is32
    /// - Size: 16x16
    /// - Format: 24-bit RGB icon
    /// - OS: System 8.5+
    IS32,
    /// - OSName: il32
    /// - Size: 32x32
    /// - Format: 24-bit RGB icon
    /// - OS: System 8.5+
    IL32,
    /// - OSName: ih32
    /// - Size: 48x48
    /// - Format: 24-bit RGB icon
    /// - OS: System 8.5+
    IH32,
    /// - OSName: it32
    /// - Size: 128x128
    /// - Format: 24-bit RGB icon
    /// - OS: Mac OS X 10.0+
    IT32,
    /// - OSName: s8mk
    /// - Size: 16x12
    /// - Format: 8-bit mask
    /// - OS: System 8.5+
    S8MK,
    /// - OSName: l8mk
    /// - Size: 32x32
    /// - Format: 8-bit mask
    /// - OS: System 8.5+
    L8MK,
    /// - OSName: h8mk
    /// - Size: 48x48
    /// - Format: 8-bit mask
    /// - OS: System 8.5+
    H8MK,
    /// - OSName: t8mk
    /// - Size: 128x128
    /// - Format: 8-bit mask
    /// - OS: Mac OS X 10.0+
    T8MK,
    /// - OSName: ic04
    /// - Size: 16x16
    /// - Format: ARGB
    /// - OS: N/A
    IC04,
    /// - OSName: ic05
    /// - Size: 32x32
    /// - Format: ARGB
    /// - OS: N/A
    IC05,
    /// - OSName: ic07
    /// - Size: 128x128
    /// - Format: PNG
    /// - OS: Mac OS X 10.7+
    IC07,
    /// - OSName: ic08
    /// - Size: 256x256
    /// - Format: PNG
    /// - OS: Mac OS X 10.5+
    IC08,
    /// - OSName: ic09
    /// - Size: 512x512
    /// - Format: PNG
    /// - OS: Mac OS X 10.5+
    IC09,
    /// - OSName: ic10
    /// - Size: 1024x1024
    /// - Format: PNG
    /// - OS: Mac OS X 10.7+
    IC10,
    /// - OSName: ic11
    /// - Size: 32x32
    /// - Format: PNG
    /// - OS: Mac OS X 10.8+
    IC11,
    /// - OSName: ic12
    /// - Size: 64x64
    /// - Format: PNG
    /// - OS: Mac OS X 10.8+
    IC12,
    /// - OSName: ic13
    /// - Size: 256x256
    /// - Format: PNG
    /// - OS: Mac OS X 10.8+
    IC13,
    /// - OSName: ic14
    /// - Size: 512x512
    /// - Format: PNG
    /// - OS: Mac OS X 10.8+
    IC14,
    /// - OSName: icp4
    /// - Size: 16x16
    /// - Format: PNG
    /// - OS: Mac OS X 10.7+
    ICP4,
    /// - OSName: icp5
    /// - Size: 32x32
    /// - Format: PNG
    /// - OS: Mac OS X 10.7+
    ICP5,
    /// - OSName: icp6
    /// - Size: 64x64
    /// - Format: PNG
    /// - OS: Mac OS X 10.7+
    ICP6,
}

impl IconFormats {
    /// Get the default recommended format for the icon type.
    pub fn recommended() -> Vec<IconFormats> {
        vec![
            IconFormats::IS32,
            IconFormats::IL32,
            IconFormats::IH32,
            IconFormats::IT32,
            IconFormats::S8MK,
            IconFormats::L8MK,
            IconFormats::H8MK,
            IconFormats::T8MK,
            IconFormats::IC04,
            IconFormats::IC05,
            IconFormats::IC07,
            IconFormats::IC08,
            IconFormats::IC09,
            IconFormats::IC10,
            IconFormats::IC11,
            IconFormats::IC12,
            IconFormats::IC13,
            IconFormats::IC14,
        ]
    }

    pub fn get_format(&self) -> FileFormat {
        match self {
            IconFormats::IS32 => FileFormat::RGB,
            IconFormats::IL32 => FileFormat::RGB,
            IconFormats::IH32 => FileFormat::RGB,
            IconFormats::IT32 => FileFormat::RGB,
            IconFormats::S8MK => FileFormat::MASK,
            IconFormats::L8MK => FileFormat::MASK,
            IconFormats::H8MK => FileFormat::MASK,
            IconFormats::T8MK => FileFormat::MASK,
            IconFormats::IC04 => FileFormat::ARGB,
            IconFormats::IC05 => FileFormat::ARGB,
            IconFormats::IC07 => FileFormat::PNG,
            IconFormats::IC08 => FileFormat::PNG,
            IconFormats::IC09 => FileFormat::PNG,
            IconFormats::IC10 => FileFormat::PNG,
            IconFormats::IC11 => FileFormat::PNG,
            IconFormats::IC12 => FileFormat::PNG,
            IconFormats::IC13 => FileFormat::PNG,
            IconFormats::IC14 => FileFormat::PNG,
            IconFormats::ICP4 => FileFormat::PNG,
            IconFormats::ICP5 => FileFormat::PNG,
            IconFormats::ICP6 => FileFormat::PNG,
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            IconFormats::IS32 => 16,
            IconFormats::IL32 => 32,
            IconFormats::IH32 => 48,
            IconFormats::IT32 => 128,
            IconFormats::S8MK => 16,
            IconFormats::L8MK => 32,
            IconFormats::H8MK => 48,
            IconFormats::T8MK => 128,
            IconFormats::IC04 => 16,
            IconFormats::IC05 => 32,
            IconFormats::IC07 => 128,
            IconFormats::IC08 => 256,
            IconFormats::IC09 => 512,
            IconFormats::IC10 => 1024,
            IconFormats::IC11 => 32,
            IconFormats::IC12 => 64,
            IconFormats::IC13 => 256,
            IconFormats::IC14 => 512,
            IconFormats::ICP4 => 16,
            IconFormats::ICP5 => 32,
            IconFormats::ICP6 => 64,
        }
    }

    pub fn get_bytes(&self) -> [u8; 4] {
        match self {
            IconFormats::IS32 => [0x69, 0x73, 0x33, 0x32], //is32
            IconFormats::IL32 => [0x69, 0x6c, 0x33, 0x32], //il32
            IconFormats::IH32 => [0x69, 0x68, 0x33, 0x32], //ih32
            IconFormats::IT32 => [0x69, 0x74, 0x33, 0x32], //it32
            IconFormats::S8MK => [0x73, 0x38, 0x6d, 0x6b], //s8mk
            IconFormats::L8MK => [0x6c, 0x38, 0x6d, 0x6b], //l8mk
            IconFormats::H8MK => [0x68, 0x38, 0x6d, 0x6b], //h8mk
            IconFormats::T8MK => [0x74, 0x38, 0x6d, 0x6b], //t8mk
            IconFormats::IC04 => [0x69, 0x63, 0x30, 0x34], //ic04
            IconFormats::IC05 => [0x69, 0x63, 0x30, 0x35], //ic05
            IconFormats::IC07 => [0x69, 0x63, 0x30, 0x37], //ic07
            IconFormats::IC08 => [0x69, 0x63, 0x30, 0x38], //ic08
            IconFormats::IC09 => [0x69, 0x63, 0x30, 0x39], //ic09
            IconFormats::IC10 => [0x69, 0x63, 0x31, 0x30], //ic10
            IconFormats::IC11 => [0x69, 0x63, 0x31, 0x31], //ic11
            IconFormats::IC12 => [0x69, 0x63, 0x31, 0x32], //ic12
            IconFormats::IC13 => [0x69, 0x63, 0x31, 0x33], //ic13
            IconFormats::IC14 => [0x69, 0x63, 0x31, 0x34], //ic14
            IconFormats::ICP4 => [0x69, 0x63, 0x70, 0x34], //icp4
            IconFormats::ICP5 => [0x69, 0x63, 0x70, 0x35], //icp5
            IconFormats::ICP6 => [0x69, 0x63, 0x70, 0x36], //icp6
        }
    }
}
