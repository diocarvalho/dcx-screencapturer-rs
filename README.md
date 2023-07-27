# screencapturer-rs

This library was made to capture screens, from the operating systems:
### Windows - work; 
### Linux - coming soon; 
### MacOs - coming soon; 

# Example code: #

```Rust
extern crate image;
extern crate dcx_screencapturer;

//used to receive buffer and save to selected format
use image::{ImageBuffer, Rgb, ImageFormat};
use dcx_screencapturer::capture_screen;

fn main() {
    //Gets the ImageBuffer<Rgb<u8>, Vec<u8>> using this parameters
    //width, height, monitor_index
    let buffer = capture_screen( 1920, 1080, 0);
    let image_buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(1920, 1080, buffer)
    .expect("Error converting to ImageBuffer");
    //path to save image file
    let image_path = "capture.png";
    //saving with png format
    image_buffer.save_with_format(image_path, ImageFormat::Png)
    .expect("Error saving image");
    print!("Image salved in path : {image_path}")
}
```
Just add the libraries on _**Cargo.toml**_

```
[dependencies]
image = "0.24.6"
dcx_screencapture = "0.1.1"
```
