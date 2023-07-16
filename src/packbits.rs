/// To denote that a byte is repeated, the first byte of a sequence
/// must be greater or equal to 128. A byte is 255 so because of this
/// 255 - 128 = 127 is the maximum amount of bytes that can be repeated.
/// When a byte is repeated, it is repeated at least 3 times.
/// So add 3 to the maximum amount of bytes that can be repeated.
const MAX_REPEAT: usize = 130;
const ENCODE_REPEAT: u8 = 128;

/// # ICNS PackBits(like) compression
/// Apple uses a format simular to PackBits to compress the image data.
/// PackBits is a lossless compression format that is used in TIFF files
/// since system 6.0.5.
/// This implementation is based on the javascript implementation by
/// @fiahfy/packbits https://github.com/fiahfy/packbits
///
/// ```rust
/// let data = vec![
///     0x01, 0x02, 0x02, 0x03, 0x03, 0x03, 0x04, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x05, 0x05
/// ];
///
/// let compressed = icns_rs::packbits::compress(data.into_boxed_slice());
///
/// assert_eq!(
///     compressed,
///     vec![0x02, 0x01, 0x02, 0x02, 0x80, 0x03, 0x81, 0x04, 0x82, 0x05]
///         .into_boxed_slice()
/// );
pub fn compress(raw: Box<[u8]>) -> Box<[u8]> {
    let mut buffers: Vec<Box<[u8]>> = vec![];

    // I'd be happy to use a iterator here
    // FIXME: This is a mess
    let mut i = 0;
    while i < raw.len() {
        let byte = &raw[i];
        // Check if last 1 or 2 bytes
        if i + 2 >= raw.len() {
            let length = raw.len() - i;
            let mut buffer = Vec::with_capacity(1);
            buffer.push(length as u8 - 1);
            buffers.push(buffer.into_boxed_slice());
            buffers.push(raw[i..].to_vec().into_boxed_slice());
            break;
        }

        // Should be repeated if the next 2 bytes are the same
        let should_repeat = byte == &raw[i + 1] && byte == &raw[i + 2];

        if should_repeat {
            let mut repeat_to = i + 2;

            while repeat_to + 1 < raw.len()
                && byte == &raw[repeat_to + 1]
                && repeat_to - i + 1 < MAX_REPEAT
            {
                repeat_to += 1;
            }

            repeat_to += 1;

            let length = repeat_to - i; // + 1 because the first byte is also included

            let mut buffer = Vec::with_capacity(2);
            buffer.push(length as u8 - 3 + ENCODE_REPEAT);
            buffer.push(byte.clone());

            buffers.push(buffer.into_boxed_slice());

            // Skip the repeated bytes
            i = repeat_to;
        } else {
            // Should not be repeated
            let mut buffer_to = i + 2;
            // ^^ Minimum length is 2 (that's why we check if we're at the last 2 bytes)
            let mut repeats = 1;
            let mut repeat_index = buffer_to;

            while buffer_to + 1 < raw.len() && buffer_to - i + 1 < ENCODE_REPEAT as usize {
                if &raw[buffer_to] == &raw[repeat_index] {
                    repeats += 1;
                    // If we have 2 repeats, we can stop
                    // It would be better to check to compress
                    if repeats > 2 {
                        break;
                    }
                } else {
                    repeats = 1;
                    repeat_index = buffer_to;
                }

                buffer_to += 1;
            }
            buffer_to += 1;
            if repeats > 2 {
                buffer_to -= 3;
            }

            let length = buffer_to - i;
            let mut buffer = Vec::with_capacity(length + 1);
            buffer.push(length as u8 - 1);
            buffer.extend_from_slice(&raw[i..buffer_to]);

            buffers.push(buffer.into_boxed_slice());

            i = buffer_to;
        }
    }

    // Compact the buffers into a single buffer
    let mut buffer = Vec::with_capacity(buffers.iter().map(|b| b.len()).sum());
    for b in buffers {
        buffer.extend_from_slice(&b);
    }

    buffer.into_boxed_slice()
}

/// # ICNS PackBits(like) decompression
/// Apple uses a format simular to PackBits to compress the image data.
/// PackBits is a lossless compression format that is used in TIFF files
/// since system 6.0.5.
/// This implementation is based on the javascript implementation by
/// @fiahfy/packbits https://github.com/fiahfy/packbits
///
/// The implementation was slightly modified to work because unlike the
/// PackBits format, the image format does not have an escape byte of
/// 255 / 0xFF. I think  the author of the javascript implementation
/// forgot to remove the escape byte in the icns version.
///
/// ```rust
/// let data = vec![0x02, 0x01, 0x02, 0x02, 0x80, 0x03, 0x81, 0x04, 0x82, 0x05];
///
/// let decompressed = icns_rs::packbits::decompress(data.into_boxed_slice());
///
/// assert_eq!(
///     decompressed,
///     vec![
///         0x01, 0x02, 0x02, 0x03, 0x03, 0x03, 0x04, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x05,
///         0x05
///     ]
///     .into_boxed_slice()
/// );
/// ```
pub fn decompress(data: Box<[u8]>) -> Box<[u8]> {
    let mut buffers: Vec<Box<[u8]>> = vec![];

    // FIXME: Don't use a loop
    let mut i = 0;
    while i < data.len() {
        // We know it's compressed if the first byte is greater or equal to 128
        if data[i] >= ENCODE_REPEAT {
            // How many times the byte is repeated
            let repeats = data[i] - ENCODE_REPEAT + 3;
            // ^^ + 3 because the first byte is also included
            let byte = data[i + 1];

            let mut buffer = Vec::with_capacity(repeats as usize);
            for _ in 0..repeats {
                buffer.push(byte);
            }

            buffers.push(buffer.into_boxed_slice());

            i += 2; // Compressed bytes are always 2 bytes long
        } else {
            // Not compressed
            let length = data[i] as usize + 1;
            let mut buffer = Vec::with_capacity(length);
            buffer.extend_from_slice(&data[i + 1..i + length + 1]);

            buffers.push(buffer.into_boxed_slice());

            i += length + 1;
        }
    }

    // Compact the buffers into a single buffer
    let mut buffer = Vec::with_capacity(buffers.iter().map(|b| b.len()).sum());
    for b in buffers {
        buffer.extend_from_slice(&b);
    }

    buffer.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_RAW: [u8; 15] = [
        0x01, 0x02, 0x02, 0x03, 0x03, 0x03, 0x04, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05, 0x05, 0x05,
    ];
    const BASIC_COMPRESSED: [u8; 10] = [0x02, 0x01, 0x02, 0x02, 0x80, 0x03, 0x81, 0x04, 0x82, 0x05];

    const STRESS_REPEAT_RAW: [u8; 131] = [0x01; 131];
    const STRESS_REPEAT_COMPRESSED: [u8; 4] = [0xFF, 0x01, 0x00, 0x01];

    const STRESS_NO_REPEAT_RAW: [u8; 131] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c,
        0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
        0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a,
        0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59,
        0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
        0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77,
        0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x80, 0x81, 0x82,
    ];
    const STRESS_NO_REPEAT_COMPRESSED: [u8; 133] = [
        0x7f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
        0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
        0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
        0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a,
        0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49,
        0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58,
        0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
        0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
        0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e, 0x7f, 0x02, 0x80, 0x81, 0x82,
    ];

    #[test]
    fn compress_basic() {
        assert_eq!(
            compress(BASIC_RAW.to_vec().into_boxed_slice()),
            BASIC_COMPRESSED.to_vec().into_boxed_slice()
        );
    }

    #[test]
    fn compress_stress_repeat() {
        assert_eq!(
            compress(STRESS_REPEAT_RAW.to_vec().into_boxed_slice()),
            STRESS_REPEAT_COMPRESSED.to_vec().into_boxed_slice()
        );
    }

    #[test]
    fn compress_stress_no_repeat() {
        assert_eq!(
            compress(STRESS_NO_REPEAT_RAW.to_vec().into_boxed_slice()),
            STRESS_NO_REPEAT_COMPRESSED.to_vec().into_boxed_slice()
        );
    }

    #[test]
    fn decompress_basic() {
        assert_eq!(
            decompress(BASIC_COMPRESSED.to_vec().into_boxed_slice()),
            BASIC_RAW.to_vec().into_boxed_slice()
        );
    }

    #[test]
    fn decompress_stress_repeat() {
        assert_eq!(
            decompress(STRESS_REPEAT_COMPRESSED.to_vec().into_boxed_slice()),
            STRESS_REPEAT_RAW.to_vec().into_boxed_slice()
        );
    }

    #[test]
    fn decompress_stress_no_repeat() {
        assert_eq!(
            decompress(STRESS_NO_REPEAT_COMPRESSED.to_vec().into_boxed_slice()),
            STRESS_NO_REPEAT_RAW.to_vec().into_boxed_slice()
        );
    }
}
