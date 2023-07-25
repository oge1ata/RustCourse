use bootloader_x86_64_common::framebuffer;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::InterruptDescriptorTable;
static mut SHOULD_EXIT_OS: bool = false;
use crate::FRAME_BUFFER;
use crate::print;
use crate::writer::FrameBufferWriter;
use alloc::string::String;
use crate::println;//use your custom println macro.

/*In this section we define handlers for interrupts*/
//1. breakpoint_handler - handles the invocation of INT3
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n Stack Frame:\n {:#?}", stack_frame);
}

//2. double_fault_handler
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n Stack Frame:\n{:#?}", stack_frame);
}

//3. General protection handler
extern "x86-interrupt" fn general_protection_handler(
    stack_frame: InterruptStackFrame, _error_code: u64)
{
    println!("EXCEPTION: GENERAL PROTECTION\n Error Code: {:#?}\n Stack Frame:\n{:#?}", _error_code, stack_frame);
}

//4. Invalid opcode handler
extern "x86-interrupt" fn invalid_opcode_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: INVALID OPCODE\n Stack Frame:\n {:#?}", stack_frame);
}


/*Here we setup our Programmable Interrupt Controller
Ref: Class slides and https://os.phil-opp.com/hardware-interrupts*/

use pic8259::ChainedPics;
use spin;

//set the offset of the pics
const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

//initialize PICS
fn init_pics(){
    unsafe { PICS.lock().initialize() };
}
//At this point, calling init_pics() from init() below 
//will not yet lead to any interrupts because the interrupt
//enable flag is unset by default.
//To enable interrupt, add x86_64::instructions::interrupts::enable();
// to the init below

//Add enum for hardware interrupt offset index
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,//offset 0 is reserved for timer
    Keyboard
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
//Add a handler for Timer
extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    //print!("."); //You can uncomment this to see that timer interrupt is on.
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

//Add a handler for keyboard
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;
    use pc_keyboard::KeyCode as Key;


    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                // DecodedKey::RawKey(key) => print!("{:?}", key),
                DecodedKey::RawKey(key) => {
                    match key {
                        Key::ArrowUp => {
                            // FrameBufferWriter.lock
                            // FRAME_BUFFER.lock().unwrap().move_cursor_down();
                            let mut frame_buffer_writer = FRAME_BUFFER.lock();
                            if let Some(writer) = frame_buffer_writer.as_mut() {
                                writer.move_cursor_up();
                            }
                            // Handle the Arrow Up functionality here
                            // println!("\u{1B}[1A");                            
                        }
                        Key::ArrowDown => {
                            // Handle the Arrow Down functionality here
                            // println!("Arrow Down");
                            let mut frame_buffer_writer = FRAME_BUFFER.lock();
                            if let Some(writer) = frame_buffer_writer.as_mut() {
                                writer.move_cursor_down();
                            }
                        }
                        Key::ArrowLeft => {
                            // Handle the Arrow Left functionality here
                            // println!("Arrow Left");
                            let mut frame_buffer_writer = FRAME_BUFFER.lock();
                            if let Some(writer) = frame_buffer_writer.as_mut() {
                                writer.move_cursor_left();
                            }
                        }
                        Key::ArrowRight => {
                            // Handle the Arrow Right functionality here
                            // println!("Arrow Right");
                            let mut frame_buffer_writer = FRAME_BUFFER.lock();
                            if let Some(writer) = frame_buffer_writer.as_mut() {
                                writer.move_cursor_right();
                            }   
                        }
                        _ => {
                            // Handle other keys
                            println!("{:?}", key);
                        }
                    }
                }
                
                // DecodedKey::Unicode(character) => {
                //     print!("{}", character);
                //     if character == '\u{001B}' {
                //         // Perform actions to exit the OS here
                //         // For example, you can set a flag to indicate the OS should exit
                //         unsafe { SHOULD_EXIT_OS = true; }
                //     }
                //     else{
                //         let character_string = String::from(character);
                //         decode_string(&character_string);
                //     }
                    
                //     /*if character == '\u{005b[A}' {
                //         // Escape sequence detected
                //         if let Some(Ok('\u{005b}')) = self.get_char() {
                //             // '[' character detected
                //             if let Some(Ok('A')) = self.get_char() {
                //                 // Arrow Up functionality
                //                 // Handle the Arrow Up functionality here
                //             } else if let Some(Ok('B')) = self.get_char() {
                //                 // Arrow Down functionality
                //                 // Handle the Arrow Down functionality here
                //             } else if let Some(Ok('C')) = self.get_char() {
                //                 // Arrow Right functionality
                //                 // Handle the Arrow Right functionality here
                //             } else if let Some(Ok('D')) = self.get_char() {
                //                 // Arrow Left functionality
                //                 // Handle the Arrow Left functionality here
                //             }
                //         }
                //     }*/
                // }
            }
        }
    }

       
    /*fn decode_string(input: &str) {
        let mut keyboard = KEYBOARD.lock();
        let mut cursor_pos: usize = 0;
    
        for c in input.chars() {
            let scancode = c as u8;
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::Unicode(character) => {
                            print!("{}", character);
                            cursor_pos += 1;
                        }
                        DecodedKey::RawKey(raw_key) => {
                            match raw_key {
                                pc_keyboard::KeyCode::ArrowUp => {
                                    println!("\u{001b}[A");
                                    cursor_pos -= 1;
                                }
                                pc_keyboard::KeyCode::ArrowDown => {
                                    println!("\u{001b}[B");
                                    cursor_pos += 1;
                                }
                                pc_keyboard::KeyCode::ArrowLeft => {
                                    println!("\u{001b}[D");
                                    cursor_pos -= 1;
                                }
                                pc_keyboard::KeyCode::ArrowRight => {
                                    println!("\u{001b}[C");
                                    cursor_pos += 1;
                                }
                                _ => println!("{:?}", raw_key),
                            }
                        }
                    }
                }
            }
        }
    }*/
    
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

//setup the IDT and make entries of all the handlers
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler); 
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

//Below function to be called from init() at the bottom of this 
//this module, to init IDT.
fn init_idt(){
    IDT.load();
}

//init all interrupts
pub fn init() {
    init_idt(); //IDT
    init_pics(); //PICS
    x86_64::instructions::interrupts::enable();//enable hardware interrupts. Without handler for timer interrupt, which is on by default, there will be a double fault
}

