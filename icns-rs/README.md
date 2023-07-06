# ICNS-RS

ICNS is a file format used by Apple to store icons for macOS applications. This crate provides a simple API for reading and (and soon)writing ICNS files.

## Roadmap

- [x] Write ICNS files
- [ ] Read ICNS files

## Usage

Here's a simple example of how to read an ICNS file:

> You can find this example in `examples/encode.rs` or run it with:
> ```sh
> cargo run --example encode
> ```

```rust
use std::fs::File;
use std::io::prelude::*;
use image::open;
use icns_rs::{IcnsEncoder, IconFormats};

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
```

## License

This project is licensed under the GPLv3 license. See the [LICENSE](/LICENSE) file for more details.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## Acknowledgements

This project is heavily inspired by:
  - The Python package: [icnsutil](https://github.com/relikd/icnsutil/)
  - The JavaScript package: [@fiahfy/icns](https://github.com/fiahfy/icns/)
  - The JavaScript PackBits implementation: [@fiahfy/packbits](https://github.com/fiahfy/packbits/)
  - The Wikipedia page: [Wikipedia: Apple Icon Image Format](https://en.wikipedia.org/wiki/Apple_Icon_Image_format#Icon_types)

When I started building this, I didn't know there already was a ICNS lib for rust, but, after looking at it, it was not up to my standards because of the lack of ARGB, RGB, and, Mask support. I wanted to create a modern package that was easy to use and had a simple API. I also wanted to make sure that it was well documented and had a good test suite. I hope you enjoy using this package as much as I enjoyed making it!.