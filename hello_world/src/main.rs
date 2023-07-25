///associate greetings module with this crate
/// 
///declare external crate
extern crate hello_world_lib;

mod greetings;
// use greetings::default_greeting;
// use greetings::french;
// use greetings::spanish;

///Load them all in one line

use greetings::{default_greeting, spanish, french};
// use hello_world_lib::{greeting_from_lib, mature_multiplier};

fn main() {
    // println!("Hello, world!");
    // print!("{}", default_greeting());
    // print!("{}", spanish::default_greeting());
    // print!("{}", french::default_greeting());
    // print!("{}", hello_world_lib::greetings_from_lib())

    // hello_world_lib::run();
    // hello_world_lib::run2();
    // hello_world_lib::run3();
    // hello_world_lib::run4();
    // hello_world_lib::run5();
    // hello_world_lib::run6();
    // hello_world_lib::run7();

    // println!("{}", hello_world_lib::mature_multiplier(&[4f64]).unwrap());

    /*match hello_world_lib::mature_multiplier(&[4.9, 7.0, 1.2]) {
        Ok(val) => println!("{}", val),
        Err(error) => println!(
            "Error number {}: {}. Detail: {} ",
            error.number, error.message, error.detail
        ),
    }
    // hello_world_lib::run7();
    println!("{}", hello_world_lib::mature_multiplier(&[4.9, 7.0, 1.2]).unwrap());

    let numbers = [1.0,2.3,4.5];

    match hello_world_lib::mature_multiplier(&numbers) {
        Ok(val) => println!("{}", val),
        Err(error) => println!("Error {}: {}", error.number, error.message)
    }

    println!("{}",hello_world_lib::mature_multiplier(&numbers).unwrap());
*/
    // hello_world_lib::run8();
    // hello_world_lib::run9();
    // hello_world_lib::run10();

    // hello_world_lib::run11();
    hello_world_lib::run12();
    hello_world_lib::run13();

}
