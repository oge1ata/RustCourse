// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)] //unit testing
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub fn greetings_from_lib() -> String{
    let message = String::from("HI FROM LIBBBB");
    message
}

/*Additional by Pius to illustrate trait.

First, we will assume that 
our struct should be layed out i
n stack, which is the default

We also want a situation in which 
none of the elements are on the heap.

In this example, notice that f
or name field, we are using string 
slice with static lifetime i.e. &'static str

In this illustration, we are 
going to define a trait which will
enable us carryout some level 
of polymorphism eventhough Rust is not
strictly an OOP language
*/
/*To summarize the difference:

    A struct is a custom data structure 
    that defines the layout and behavior 
    of a specific type of object. It groups 
    related data fields together.

    A trait is a set of method declarations 
    that can be implemented by different types. 
    It defines shared behavior or functionality 
    that can be utilized across multiple types.

In essence, a struct defines the data and behavior 
of a specific type, while a trait defines a set of 
methods that can be implemented by multiple types 
to provide shared functionality. */

trait Shape {
    fn area(&self) -> f32;
    fn new(length: f32, width: f32, name: &'static str) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> &'static str;
    fn set_name(&mut self, name: &'static str);
}

/// traits are used to define a set of methods that can be implemented
/// by different types
/// Shape trait declares several methode
/// area(&self) -> f32: This method calculates and returns the area of the shape. It takes an immutable reference to self (the instance of the implementing type) and returns a 32-bit floating-point value (f32).
/// new(length: f32, width: f32, name: &'static str) -> Self: This method is a constructor for creating new instances of the implementing type. 
/// It takes three parameters: length and width (both f32 values) represent the dimensions of the shape, and name 
/// is a static string (&'static str) that represents the name of the shape. It returns an instance of Self, which represents the implementing type itself.
// get_name(&self) -> &'static str: This method returns the name of the shape. It takes an immutable reference to self and returns the name as a static string (&'static str).
/// set_name(&mut self, name: &'static str): This method sets the name of the shape. It takes a mutable reference to self and a name parameter of type &'static str.

#[derive(Debug, Clone)]
struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}
/*Additionally, the #[derive(Debug, Clone)] attribute is used above the struct declaration. This attribute is used to automatically generate implementations for the Debug and Clone traits for the Rect struct.

    Debug: The Debug trait enables the struct 
    to be formatted and printed using the {:?}
     format specifier. It allows you to print
      the Rect struct and its fields for
       debugging purposes.

    Clone: The Clone trait provides a way to 
    create a new copy of an existing Rect
     object. This can be useful when you
      need to create a new instance with the same
       field values as an existing instance.

By deriving these traits, you get default implementations for methods like fmt::Debug::fmt (for printing the struct) and clone (for creating a copy of the struct). */

/*
The main difference between &'static str and str is the 
lifetime. A regular str reference can have different lifetimes
 depending on the context, whereas &'static str
  explicitly represents a string reference with a static lifetime
   that lasts for the entire program execution.

It's important to note that while 'static strings 
have a longer lifetime, they are immutable and cannot be
 modified. If you need a mutable string,
  you would use String instead of &'static str. */

impl Rect {
    fn default() -> Self {
        Rect {
            length: 1f32,
            width: 1f32,
            name: "default_name",
        }
    }
}

/*Associated functions in Rust are functions that are associated with a type, 
but they don't require an instance of the type to be called. They are similar to static methods in other languages.

The default() function in this case serves as a constructor for the Rect struct. 
It creates and returns a new instance of Rect with default values. */

impl Shape for Rect {
    ///Associated function used to create a new Shape
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }
    ///Area method
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
}


impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        //self.length == other.length && self.width == other.width && self.name == other.name
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
///an inbuilt trait that can be called using == or !=
/// We didn't declare
/// Look below
/// assert_eq!(rect1 == rect2, false); // Not equal
///assert_eq!(rect1 != rect2, true); // Not equal


pub fn run() {
    let rectangle1 = Rect {
        length: 2.4,
        width: 6.3,
        name: "Rectangle 1",
    };
    let mut rectangle2 = Rect::default();
    rectangle2.set_length(10f32);
    rectangle2.set_width(5f32);

    let rectangle3 = rectangle1.clone();

    let rectangle4 = Rect {
        length: 12f32,
        ..rectangle1
    };

    println!("rectangle 1 is {:#?}", rectangle1);
    println!("rectangle 2 is {:#?}", rectangle2);
    println!("Area of rectangle 1 is {}", rectangle1.area());
    assert_eq!(rectangle1, rectangle3);
    assert_ne!(rectangle4, rectangle1); //no output if true
    println!("If you can see this, your two triangles are equal");
}

///let the string element be on the heap
trait Shape2: core::fmt::Debug {
    fn area(&self) -> f32;
    //fn new (length: f32, width: f32, name: &str) -> Self;
    fn get_length(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_width(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: &str);
}
#[derive(Debug, Clone)]
struct Rect2 {
    length: f32,
    width: f32,
    name: String, //on heap
}

impl Rect2 {
    fn default() -> Self {
        Rect2 {
            length: 1f32,
            width: 1f32,
            //name: "default_name".to_string()
            name: String::from("default_name"),
        }
    }
}

impl Shape2 for Rect2 {
    ///Area method
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

///Implement a From trait for Rect2 
/// that takes a string slice
/// with the format "length,width,name"
/// The From trait is a generic trait in 
///  that provides a conversion mechanism between types. It allows you to define how one type can be converted into another type.

///In this case, the implementation specifies how 
/// a &str can be converted into a Rect2 instance.
impl From<&str> for Rect2 {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };

        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0f32,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",

        };
        Rect2 {
            length,
            width,
            name: name.to_owned(),
            //Creates owned data from borrowed data, usually by cloning.


        }
        
    }
}
/*returns a 
/// value for some and for none 
/// sets an empty string.*/
    
//Implement Into trait for Rect2
impl Into<String> for Rect2 {
    fn into(self) -> String {
        //Let's return a string template literal
        format!("My name is {} and my area is {}.", self.name, self.area())
    }
}

/*The code you provided is an implementation block 
(impl) for the Into<String> trait on the Rect2 struct.
 It specifies how an instance of Rect2 can be converted into a String.

The Into trait is a generic trait in Rust that provides a conversion mechanism from one type to another. 
It is the reciprocal of the From trait. It allows you to define 
how a type can be converted into another type. */

//Let's demand that our structs be created on the heap

pub fn run2() {
    let rectangle1: Box<dyn Shape2> = Box::new(Rect2 {
        length: 12f32,
        width: 9f32,
        name: "Rectangle 1".to_owned(),
    });

    let rectangle2 = Rect2::from("20.0,30.0,Rectangle2");
    let rectangle3: Rect2 = "25.0,37.0,Rectangle3".into(); //this is the reverse of From

    let rectangle4 = Rect2::default();
    let s: String = rectangle3.into(); //this is the Info<String> trait impl. Explicit type declaration. This is a move
    println!("About me: {}", s);
    println!("Rectangle4 = {:#?}", rectangle4);
    //println!("Area of Rectangle 3 = {}",rectangle3.area()); //moved before now. So, will not work

    println!("Rectangle 1 = {:#?}", rectangle1);
    println!("Area of Rectangle 2 = {}", rectangle2.area());
}
/* The into function takes ownership of the string literal, applies the conversion logic defined in the Into<Rect2> implementation, 
and returns a Rect2 instance.
The resulting Rect2 instance is assigned to rectangle3 with an explicit type annotation of Rect2.

The "20.0,30.0,Rectangle2" is a string literal that is being passed as an argument to the from function.
The function parses the string and initializes the fields of Rect2 accordingly.
The resulting Rect2 instance is assigned to rectangle2 using type inference.

The Box<dyn Shape2> type annotation indicates that rectangle1 is a trait object implementing the Shape2 trait. 
This allows you to store objects of different types that implement the Shape2 trait in a homogeneous container.
The Box provides ownership and manages the memory of the heap-allocated Rect2 instance.*/


///Functions and Closures
//We have been using functions already, including the main() which is the program entry point
//In this section, we are particularly highlightingn the fact that functions
//have a type unto themselves and variables of a given function type
//can be declared and passed to another function.
//So, we can have a serious of function calls, the output of one becoming the input of
//the next. Herein lies the concept of higher order functions

//As already mentioned, in Rust, functions have their own types.
//Below is an illustration

///Function to add to two signed integers. Returns a signed integer
fn add(a: i32, b: i32) -> i32 {
    a + b
}
//The function type embodied in the above is fn(i32, i32) -> i32.
//Function type is defined by the keyword fn followed by the optional expected parameter types
//and then the optional expected return type.

///Here we define a function name apply that is expected to receive the function type
/// above name f here, along with two other unsigned interger parameters named x and y
/// respectively
fn apply(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y) //a call to the function passed, which in its turn is passed the two other parameters
}

pub fn run3() {
    let f = add;
    let x = 7;
    let y = 8;
    let z = apply(f, x, y);
    println!("The result of applying f to {} and {} is {}", x, y, z);
}

///let's define another function that handles straight line graph formula
///Assuminng that m, c and x have to be passed.
///Here you can use a normal function.
///Below, we have to use array slice as x, otherwise, we will need to specify a size.

fn straight_line_function(m: i32, c: i32, xses: &[i32]) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
    for x in xses {
        let y = (m * x) + c;
        output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
    }
    output
}
/*    Closures are anonymous functions that can capture values from their surrounding environment. They are defined using the |...| { ... } syntax.
    Closures can access variables from the surrounding environment, even after they have gone out of scope. This is known as "capturing" variables.
    In contrast, functions do not capture values from the environment by default. They can only operate on the values passed to them as arguments.
    Functions are separate entities and can be defined independently, while closures are defined at the location where they are used.
    Closures are useful when you need to create a function-like behavior that depends on variables from the surrounding environment, allowing for flexibility and concise code.
    Functions, on the other hand, provide a clear separation of concerns and can be reused in different contexts without being tied to the surrounding environment.

In the code example, the closure strait_line_closure attempts to access the variables m and c from its surrounding environment. 
However, since closures capture variables by reference or by value, and cannot capture variables that are borrowed by mutable 
references (such as m and c in this case), it will result in a compilation error. 
Functions, in contrast, can access variables passed as arguments, allowing the straight_line_function to work as intended.

To summarize, closures are more flexible in capturing values from the surrounding environment, while functions provide a clear separation and can be reused independently. */

//Let's address Closure
//What if m and y are declared outside the apply function above
//and we pass only x. E.g.,

pub fn run4() {
    let c = 10;
    let m = 20;
    let xses = [1, 2, 3, 4, 5];

    //Let's use our straight_line function above. We must pass m,c and xses as arguments
    let output = straight_line_function(m, c, &xses);
    println!("Points for straight line plot are {:?}", output);

    //Let us use closure without having to pass m and c
    let strait_line_closure = |xses: &[i32]| -> Vec<(i32, i32)> {
        let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
        for x in xses {
            let y = (m * x) + c;
            output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
        }
        output
    };

    let output2 = strait_line_closure(&xses); //Can read m an y from the environment

    println!("Points for straight line plot 2 are {:?}", output2);

    //Below should report - can't capture dynamic environment in a fn item
    //m and c must be passed
    /*
    fn strait_line_function2(xses: &[i32]) -> Vec<(i32, i32)> {
        let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
        for x in xses {
            let y = (m * x) + c;
            output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
        }
        output
    };
    */
}

pub fn run5() {
    /*let's see closures in action
    We will map through a collection, square each value, retain only odd numbers and sum them or collect them
    */
    let my_array = [1, 2, 3, 4, 5, 6, 7];

    //data types in stack implement Copy trait by default, so I can use my_array twice below.
    let sum_of_all_even_numbers_after_square: i32 = my_array
        .into_iter()
        .map(|n| n ^ 2)
        .filter(|n| n % 2 == 0)
        .sum();

    let collection_of_all_even_numbers_after_square: Vec<i32> = my_array
        .into_iter()
        .map(|n| n ^ 2)
        .filter(|n| n % 2 == 0)
        .collect();

    println!(
        "Sum of all even numbers in array after square = {}.",
        sum_of_all_even_numbers_after_square
    );

    println!(
        "Collection of all even numbers in array after square = {:?}.",
        collection_of_all_even_numbers_after_square
    );

    //Tuples have no in-build iterator, so destructure first into vector
    let my_tuple = (1, 2, 3, 4, 5, 6, 7);
    let (a, b, c, d, e, f, g) = my_tuple;
    let my_vec = vec![a, b, c, d, e, f, g];

    //clone is used below so that I can reuse my_vec again. Vec do not implement Copy trait by default.
    let sum_of_all_even_numbers_from_tuple_after_square: i32 = my_vec
        .clone()
        .into_iter()
        .map(|n| n ^ 2)
        .filter(|n| n % 2 == 0)
        .sum();

    let collection_of_all_even_numbers_from_tuple_after_square: Vec<i32> = my_vec
        .into_iter()
        .map(|n| n ^ 2)
        .filter(|n| n % 2 == 0)
        .collect();

    println!(
        "Sum of all even numbers from tuple after square = {}.",
        sum_of_all_even_numbers_from_tuple_after_square
    );

    println!(
        "Collection of all even numbers from tuple after square = {:?}.",
        collection_of_all_even_numbers_from_tuple_after_square
    );
}

pub fn run6() {
    let mut x = 10;

    println!("x before change = {}", x);

    let y = &mut x; //y is a mutable reference to x
    let z: *const u32 = y; //z is an immutable raw pointer to y which references x
                           //let a = y as *mut u32; //a is a mutable raw pointer to y which references x
    let a: *mut u32 = y; //a is a mutable raw pointer to y which references x. Same as previous line

    println!("y = {:?}", y); //expect value in x
    println!("z = {:?}", z); //expect memory address
    println!("a = {:?}", a); //expect same memory address as z above

    *y = 11; //expect value in x to change
    println!("x after first change = {}", x);

    unsafe {
        *a = 12; //expect value in x to change
        assert!(x == 12)
    };

    println!("x after second change = {}", x);
}

pub fn run7() {
    //Error handling
    //See my slides for references
    //panic!("Problem. You called panic");

    //Illustrate Some
    let mut v= vec!["a", "b", "c"];

    //pop an element from the vector
    let x = v.pop();

    //println!("{}", x.expect("I expected a value from my vector. You messed up!"));

    //What if we know that there is a possibility of having no Some value
    match x {
        Some(value) => println!("Popped {}", value),
        None => println!("Your vector is empty"),
    }
    //compare above to:
    let mut v2: Vec<&str> = Vec::new();
    // let mut v2: Vec<&str> = vec!["a", "2", "3"];

    //let mut y2 = v2.pop().unwrap(); //will panic without message because it returns an unhandled None
    //let mut y2 = v2.pop().expect("Do not call pop on an empty Vector"); //will panic and send a message

    //Exercise: How can you ensure that your program does not panic when you call a function that returns an Option?
    let y2 = match v2.pop() {
        Some(val) => val,
        None => "Empty vector",
    };
    println!("{}", y2);

    //let's use ? for Option
    let mut v3 = vec![1, 2, 3];

    let mut plus_one = || -> Option<i32> { Some(v3.pop()? + 1) };

    println!("Plus one: {}", plus_one().unwrap());
}

//Let's see Result instead of Option
//Here it returns OK value vs Err, unlike Option that returns Some value vs None

//Adjust the following to return Result

pub fn multiplier(numbers: &[f64]) -> f64 {
    let mut product = 1f64;
    for n in numbers {
        product = product * n;
    }
    product
}

//What if we want to return Err to the caller of this function when less
//than two arguments are passed

#[derive(Debug)]
pub struct ErrorTypes {
    pub number: u8,
    pub message: &'static str,
    pub detail: &'static str,
}

//Let's create static variables for our error types
const INVALID_ARG_LEN_ERR: ErrorTypes = ErrorTypes {
    number: 101,
    message: "Invalid Arg Length",
    detail: "Two or more arguments are expected",
};

const INVALID_ARG_TYPE_ERR: ErrorTypes = ErrorTypes {
    number: 102,
    message: "Invalid Arg Type. f64 expected",
    detail: "Invalid Arg Type. f64 expected. You must convert your arg to f64",
};

pub fn mature_multiplier(numbers: &[f64]) -> Result<f64, ErrorTypes> {
    if numbers.len() < 2 {
        return Err(INVALID_ARG_LEN_ERR);
    };
    let mut product = 1f64;
    for n in numbers {
        product *= n;
    }
    Ok(product)
}

//ownership and borrowing illustrations
//scope
pub fn run8() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("{}", s);

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    //above so far is all about stack. What about when we are dealing with
    //heap allocated memory where for memory saving purposes, different variable can
    //share the heap allocated memory?

    {
        let s = String::from("world"); // s is valid from this point forward
        println!("{}", s);
        // do stuff with s
    } // this scope is now over, and s is no
      // longer valid

    /*
    There is a natural point at which we can return the memory our
    String needs to the allocator: when s goes out of scope.
    When a variable goes out of scope, Rust calls a special function for us.
    This function is called drop, and it’s where the author of String
    can put the code to return the memory. Rust calls drop automatically
    at the closing curly bracket.
     */

    let x = 5;
    let y = x;
    println!("{}", y);
    /*
    above shows: bind the value 5 to x; then make a copy of the value in x
    and bind it to y.” We now have two variables, x and y,
    and both equal 5
     */

    //Now let’s look at the String version:
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);
    // println!("{}, world!", s1);
    //What is happening above?
    //it says value has been borrowed or moved, so string doesn't do copying
}
/*    Stack-allocated variables:
        In the first example, the variable s is stack-allocated because its type is a string slice (&str).
        Stack-allocated values have a fixed size known at compile time and are pushed onto the stack.
        When the scope containing the variable ends (closing curly brace), the variable goes out of scope and is no longer valid.
        The memory associated with stack-allocated variables is automatically freed when they go out of scope.

    Heap-allocated variables (String type):
        In the second example, the variable s is a String type, which is a string that is stored on the heap.
        Heap-allocated values have a flexible size and are stored on the heap, allowing for dynamic memory allocation.
        When a String value is created with String::from, it allocates memory on the heap to store the contents of the string.
        The ownership of the heap-allocated memory is transferred to the variable s.
        When the scope containing the variable ends, the variable goes out of scope, and the memory allocated on the heap is automatically freed.
        In the example, after s goes out of scope, the memory for the string is freed.

    Ownership and copying:
        In the third part of the code, the variable x is a stack-allocated i32, which implements the Copy trait.
        Types that implement the Copy trait are copied rather than moved when assigned to another variable.
        When the value of x is assigned to y, a bitwise copy of the value is made, and both variables are independent of each other.
        So, both x and y are valid and contain their respective copies of the value.
        The Copy trait is implemented for types that are simple and have a known size at compile time.

    Ownership transfer and move semantics:
        In the fourth part of the code, the variable s1 is a String type, which is a heap-allocated string.
        When s1 is assigned to s2, ownership of the heap-allocated memory is transferred from s1 to s2.
        This transfer is called a "move," and it invalidates the previous variable, s1.
        After the move, s1 is no longer valid because its ownership was transferred to s2.
        Therefore, trying to access s1 after the move would result in a compilation error.
        The move semantics ensure that there is always only one valid owner of heap-allocated memory at any given time.

In summary, ownership in Rust ensures that memory is managed correctly and efficiently, 
preventing issues like dangling pointers, double frees, and data races. 
The ownership model, along with borrowing rules, allows Rust to provide memory safety
 guarantees without the need for garbage collection or manual memory management. */

//Declarative macros
/*
Case 1: We would like to create a 
macro that would allow us instantiate
one or more rectangles (along with their Shape 
    trait impl) at a go. i.e., :
rectangles!((length:f32, width:f32, name:&str),…,n)
E.g., rectangles!((1, 1, "rect1"), (3.5, 7.0, "rect2"))
 */

 #[macro_export] //in-built in Rust
 macro_rules! rectangles {
     ($($rect_props_tuple:expr),*) => {
         //I want to return a Vector of Rectangles
         {
             let mut rect_vec = Vec::new();
             //take our expression received, get the length, width and name
             //from each and create the appropriate Rect and push each
             //to our rect_rec
             $(let (length, width, name) = $rect_props_tuple;
             let rect = Rect{length: length as f32,
                 width: width as f32, name: name as &str};
             rect_vec.push(rect);
             )*
             rect_vec
         }
 
     };
 }

 //Try our rectangles! declarative macro.
pub fn run9(){
    let rects = rectangles!((1,1,"rect1"),(3.5,7.0,"rect2"));
    println!(
        "Area of rectangle 1 = {}; area of rectangle 2 = {}",
        rects[0].area(),
        rects[1].area()
    )
}


//You can also have multiple expressions in a declarative macro.
//What if you want a second expression that contains defaults for 
//length, width and name for the rect
//This implies that length, width and name will be optional.

#[macro_export]
macro_rules! rectangles_with_default {
    (($($rect_props_tuple:expr),*), $default_tuple:expr) => {
        {
            let mut rect_vec = Vec::new();
            let (default_length, default_width, default_name) = $default_tuple;
            $(
                let (length, width, name) = $rect_props_tuple;
                let rect = Rect{
                    length: if length.is_none(){default_length as f32} else {length.unwrap() as f32},
                    width: if width.is_none() {default_width as f32} else {width.unwrap() as f32},
                    name: if name.is_none() {default_name as &str} else {name.unwrap() as &str}
                };
                rect_vec.push(rect);
            )*
            rect_vec
        }
    };
}

pub fn run10(){
    let rects = rectangles_with_default!(
        (
            (None as Option<u32>, Some(-1), Some("rect1")),
            (Some(3.5), Some(7.0), None as Option<&str>)

        ),
        (1, 1, "default_name")
    );

    println!(
        "Area of rectangle named '{}' = {}; Area of rectangle named '{}' = {}",
        rects[0].name,
        rects[0].area(),
        rects[1].name,
        rects[1].area()
    )
}

pub fn run11() {
    println!("-----About to test Derive macro-----");
    use rect_shape::Shape;
    use rect_shape_derive::Shape;

    #[derive(Debug, Clone, Shape)]
    struct RectWithDerivedShape {
        length: f32,
        width: f32,
        name: &'static str,
    }

    //the Shape trait implementations
    //should be available for RectWithDerivedShape
    //without further explicit implementation

    let rectangle1 = RectWithDerivedShape {
        length: 1.0,
        width: 2.0,
        name: "Rect 1 with derived Shape trait",
    };
    println!(
        "Area of rectangle1 with derived Shape = {}",
        rectangle1.area()
    );
}

pub fn run12() {
    println!("-----About to test Attribute-like macros-----");
    //next is Attribute-like macros
    use my_attribute_proc_macros::my_attribute_macro;

    #[my_attribute_macro(10)]
    fn my_ordinary_function(x: i32) -> i32 {
        x * 3
    }

    println!("{}", my_ordinary_function(3))
}

pub fn run13() {
    println!("----About to test Attribute-like macros 2----");
    //next is Attribute-like macros
    use my_attribute_proc_macros::route;

    #[route("GET", "/")]
    fn my_controller_endpoint() { 
        let header = "test header";
        header
    }

    println!("{:?}", my_controller_endpoint())
}