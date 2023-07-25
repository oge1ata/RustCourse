#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut frame_buffer_writer = $crate::FRAME_BUFFER.lock();
            if let Some(writer) = frame_buffer_writer.as_mut() {
                writer.write_fmt(format_args!($($arg)*)).unwrap();
            }
        }
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}