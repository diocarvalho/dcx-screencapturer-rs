
#![allow(unused_assignments)]
extern crate windows_sys;

pub use dcx_screencapture::capture_screen;

#[cfg(target_os = "windows")]
mod dcx_screencapture
{
    use windows_sys::Win32::Graphics::Gdi::{
        CreateCompatibleDC,
        CreateCompatibleBitmap, 
        BitBlt, 
        GetDIBits, 
        DeleteDC, 
        SelectObject, 
        GetDC, 
        ReleaseDC, 
        SRCCOPY, 
        HGDIOBJ, 
        BITMAPINFO, 
        BITMAPINFOHEADER, 
        DIB_RGB_COLORS
    };

    pub fn capture_screen(width: i32, height: i32, monitor_index: isize) -> Vec<u8>{
        let mut buffer: Vec<u8> = vec![0; (width * height * 3) as usize];

        unsafe {                
                // Receive device context
                // Params: monitor index
                let screen_dc = GetDC(monitor_index);

                // Create a memory dc
                // Params: device context
                let memory_dc = CreateCompatibleDC(screen_dc);
            
                // Create a bitmap compatible
                // Params: device context, width, height
                let bmp = CreateCompatibleBitmap(screen_dc, width as i32, height as i32);

                // Select the bitmap on context
                // Params: memory dc, bitmap size
                let old_bmp = SelectObject(memory_dc, bmp as HGDIOBJ);

                // Copy the screen picture to bitmap
                // Params: memory dc, dest startx , dest starty, dest width, dest height, device context, src startx, src starty, raster-operation code
                BitBlt(memory_dc, 0, 0, width, height, screen_dc, 0, 0, SRCCOPY);

                // Create variable to receive buffer
                let mut bmp_info: BITMAPINFO = std::mem::zeroed();
                bmp_info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
                bmp_info.bmiHeader.biWidth = width;
                // Invert height to correct image
                bmp_info.bmiHeader.biHeight = -height;
                bmp_info.bmiHeader.biPlanes = 1;
                bmp_info.bmiHeader.biBitCount = 24;
                bmp_info.bmiHeader.biCompression = 0;

                // Copy the bitmap data to buffer
                // Params: mem dc, bitmap startIndex, scan lines to retrieve, 
                // A pointer to a buffer to receive the bitmap data, A pointer to a BITMAPINFO structure,
                // The format of the bmiColors member of the BITMAPINFO structure
                GetDIBits(memory_dc, bmp, 0, height as u32, buffer.as_mut_ptr() as *mut _, &mut bmp_info, DIB_RGB_COLORS);

                //invert R with B to correct coloration
                for pixel in buffer.chunks_exact_mut(3) {
                    let tmp = pixel[0];
                    pixel[0] = pixel[2];
                    pixel[2] = tmp;
                }

                // Restore the original context
                SelectObject(memory_dc, old_bmp);
                // Delete memory dc
                DeleteDC(memory_dc);
                // Release device context
                ReleaseDC(0, screen_dc);

        }
        return buffer;
    }
}
