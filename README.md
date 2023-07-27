# screencapturer-rs

This library was made to capture screens, from the operating systems:
### Windows - work; 
### Linux - coming soon; 
### MacOs - coming soon; 

# Example code: #

```
extern crate image;
extern crate screencapture;

//used to receive buffer and save to selected format
use image::{ImageBuffer, Rgb, ImageFormat};
use screencapture::capture_screen;

fn main() {
    //Gets the ImageBuffer<Rgb<u8>, Vec<u8>> using this parameters
    //width, height, monitor_index
    let image_buffer = capture_screen( 1920, 1080, 0);
    //path to save image file
    let image_path = "capture.png";
    //saving with png format
    image_buffer.save_with_format(image_path, ImageFormat::Png).expect("error saving image");
    print!("Image salved in path : {image_path}")
}
```
Just add the libraries on _**Cargo.toml**_

```
[dependencies]
image = "0.24.6"
screencapture = "0.1.1"
```
