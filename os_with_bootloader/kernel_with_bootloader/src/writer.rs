//writer.rs
mod constants;

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use constants::font_constants;
use constants::font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT, SCREEN_HEIGHT, SCREEN_WIDTH};
use core::{
    fmt::{self, Write},
    ptr,
};
use alloc::string::String;
use lazy_static::lazy_static;
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};
use spin::Mutex;

lazy_static! {
    pub static ref FRAME_BUFFER_WRITER: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);
}

// // lazy_static! {
// //     pub static ref FRAME_BUFFER_WRITER: Mutex<FrameBufferWriter> = {
// //         // Create a static framebuffer array with a fixed size
// //         static mut FRAME_BUFFER: [u8; 100] = [100; 100];

// //         // Get a mutable reference to the framebuffer
// //         let framebuffer_ptr: *mut [u8; 100] = unsafe { &mut FRAME_BUFFER };

// //         // Create the FrameBufferWriter using the framebuffer reference
// //         Mutex::new(FrameBufferWriter {
// //             framebuffer: unsafe { &mut *framebuffer_ptr },
// //             info: FrameBufferInfo {
// //                 width: 400,  // Set the width of your framebuffer
// //                 height: 200, // Set the height of your framebuffer
// //                 stride: 0,   // Set the stride of your framebuffer
// //                 pixel_format: PixelFormat::Rgb, // Set the pixel format of your framebuffer
// //                 bytes_per_pixel: 40, // Set the number of bytes per pixel in your framebuffer
// //                 byte_len: 80,
// //             },
// //             x_pos: 100,
// //             y_pos: 200,
// //         })
// //     };
// // }

// #[macro_export]
// macro_rules! print {
//     ($($arg:tt)*) => {$crate::writer::_print(format_args!($($arg)*));};
// }

// #[macro_export]
// macro_rules! println {
//     () => ($crate::print!("\n"));
//     ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
// }

// #[doc(hidden)]
// pub fn _print(args: fmt::Arguments) {
//     // use core::fmt::Write;
//     FRAME_BUFFER_WRITER.lock().as_mut().unwrap().write_fmt(args).unwrap();}

/// Additional vertical space between lines
const LINE_SPACING: usize = 2;

/// Additional horizontal space between characters.
const LETTER_SPACING: usize = 0;

struct TextBuffer {
    text: String,
}

impl TextBuffer {
    fn new() -> TextBuffer {
        TextBuffer {
            text: String::new(),
        }
    }

    fn get_text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, new_text: String) {
        self.text = new_text;
    }
}

/// Padding from the border. Prevent that font is too close to border.
const BORDER_PADDING: usize = 1;

/// Returns the raster of the given char or the raster of [`font_constants::BACKUP_CHAR`].
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

/// Allows logging text to a pixel-based framebuffer.
pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl FrameBufferWriter {
    /// Creates a new logger that uses the given framebuffer.

    /// Creates a new logger that uses the given framebuffer.
    pub fn new(
        framebuffer: &'static mut [u8],
        info: FrameBufferInfo,
        column: usize,
        row: usize,
    ) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
        };
        logger.clear();
        logger.move_horizontal(row);
        logger.move_vertical(column);
        logger
    }

    // pub fn newnew(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
    //     let frame_buff_write = FrameBufferWriter::new(framebuffer, info);
    //     if FRAME_BUFFER_WRITER.lock().is_none()
    //     {
    //         *FRAME_BUFFER_WRITER.lock() = Some(frame_buff_write);
    //     }
    // }

    fn newline(&mut self) {
        self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    /// Erases all text on the screen. Resets `self.x_pos` and `self.y_pos`.
    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.fill(0);
    }

    pub fn move_cursor_up(&mut self){
        if self.y_pos > 0 {
            self.y_pos -= font_constants::CHAR_RASTER_HEIGHT.val();
        }
    }

    pub fn move_cursor_down(&mut self){
        if self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() < SCREEN_HEIGHT.into() {
        self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val();
    }
    }

    pub fn move_cursor_left(&mut self){
        if self.x_pos > BORDER_PADDING + font_constants::CHAR_RASTER_WIDTH 
        {
            self.x_pos -= font_constants::CHAR_RASTER_WIDTH;
        }
        // if self.x_pos + font_constants::CHAR_RASTER_WIDTH < SCREEN_WIDTH - BORDER_PADDING {
            //     self.x_pos += font_constants::CHAR_RASTER_WIDTH;
    }
    
    pub fn move_cursor_right(&mut self){
        
        // if self.x_pos + font_constants::CHAR_RASTER_WIDTH < SCREEN_WIDTH - BORDER_PADDING {
        //         self.x_pos += font_constants::CHAR_RASTER_WIDTH;}
    }

    

    /// Writes a single char to the framebuffer. Takes care of special control characters, such as
    /// newlines and carriage returns.
    fn write_char(&mut self, c: char) {
        // let mut text_buffer = TextBuffer::new();
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            '\t' => {
                // Handle tab logic
                // For example, you can insert a tab character ('\t') at the current cursor position:
                let tab_width = 5; // Define the desired width of a tab
                for _ in 0..tab_width {
                    // Write a tab character to the buffer
                    // self.framebuffer.write_char('\t');
                    // Move the cursor position
                    self.x_pos += font_constants::CHAR_RASTER_WIDTH;
                }},
            
            '\u{0008}' => {
                // Backspace
                // if self.x_pos >= BORDER_PADDING+ font_constants::CHAR_RASTER_WIDTH{
                //     self.x_pos -= font_constants::CHAR_RASTER_WIDTH;
                //     self.write_pixel(self.x_pos, self.y_pos, 0);

                //     // Delete the character from the string
                //     let mut new_text = String::new();
                //     let current_text = text_buffer.get_text(); // Get the current text string
                //     if current_text.len() > 0 {
                //         new_text.push_str(&current_text[..current_text.len() - 1]);
                //     }
                //     text_buffer.set_text(new_text); // Set the updated text string
                // }

                //role models code
                if self.x_pos >= BORDER_PADDING+ font_constants::CHAR_RASTER_WIDTH{
                    //move the cursor back twice because mine has moe spacing
                    self.x_pos -= font_constants::CHAR_RASTER_WIDTH;
                    // self.x_pos -= font_constants::CHAR_RASTER_WIDTH;
                    //overried the chcarcter that was there with a space
                    for y in self.y_pos..(self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val()){
                        for x in self.x_pos..(self.x_pos + font_constants::CHAR_RASTER_WIDTH){
                            self.write_pixel(x, y, 0);
                        }
                    }

                    if self.x_pos == BORDER_PADDING {
                        self.y_pos -= font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
                        self.x_pos = self.width()-BORDER_PADDING-font_constants::CHAR_RASTER_WIDTH - LETTER_SPACING;
                        // self.x_pos = self.width()-font_constants::CHAR_RASTER_WIDTH;
                    }

                    // Move the cursor to the previous line
                    // self.y_pos -= font_constants::CHAR_RASTER_HEIGHT.val();
               }
            }

            // _ => {
            //     match key {
            //         // Arrow Up
            //         "\u{001b}[A" => {
            //             // Handle the Arrow Up functionality here
            // if self.y_pos > 0 {
            //     self.y_pos -= font_constants::CHAR_RASTER_HEIGHT.val();
            // }
            //         }
            //         // Arrow Down
            //         "\u{001b}[B" => {
            //             if self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() < SCREEN_HEIGHT {
    //     self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val();
    // }
            //         }
            //         // Arrow Left
            //         "\u{001b}[D" => {
        //     //             if self.x_pos > BORDER_PADDING + font_constants::CHAR_RASTER_WIDTH {
        // self.x_pos -= font_constants::CHAR_RASTER_WIDTH;
            //         }
            //         // Arrow Right
            //         "\u{001b}[C" => {
            //             // Handle the Arrow Right functionality here
            // if self.x_pos + font_constants::CHAR_RASTER_WIDTH < SCREEN_WIDTH - BORDER_PADDING {
            //     self.x_pos += font_constants::CHAR_RASTER_WIDTH;
            // }
            //         }
            //         // Tab
            //         "\u{0009}" => {
            //             // Handle the Tab functionality here
            //         }
            //         _ => {
            //             let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
            //             if new_xpos >= self.width() {
            //                 self.newline();
            //             }
            //             let new_ypos =
            //                 self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
            //             if new_ypos >= self.height() {
            //                 self.clear();
            //             }
            //             self.write_rendered_char(get_char_raster(c));
            //             self.set_position(self.x_pos + font_constants::CHAR_RASTER_WIDTH, self.y_pos);
            //         }
            //     }
            // }    
            

                       
            c => {
                let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= self.width() {
                    self.newline();
                }
                let new_ypos =
                    self.y_pos + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= self.height() {
                    self.clear();
                }
                self.write_rendered_char(get_char_raster(c));
                self.set_position(self.x_pos, self.y_pos); //basically doubles it
            }
        }
        
    }

    /// Prints a rendered char into the framebuffer.
    /// Updates `self.x_pos`.
    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => {
                // set a supported (but invalid) pixel format before panicking to avoid a double
                // panic; it might not be readable though
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in logger", other)
            }
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }

    /// Sets the write position to the specified coordinates.
    pub fn set_position(&mut self, x: usize, y: usize) {
        self.x_pos = x;
        self.y_pos = y;
    }

    /// Moves the write position horizontally by the specified offset.
    pub fn move_horizontal(&mut self, offset: usize) {
        self.x_pos += offset;
    }

    /// Moves the write position vertically by the specified offset.
    pub fn move_vertical(&mut self, offset: usize) {
        self.y_pos += offset;
    }

}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}

