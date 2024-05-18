fn main() {
    // this code will fail because multiple types are possible 
    // and we need to explicitly specify the type 
    //let guess = "41".parse().expect("Not a number!");
    let guess: u32 = "41".parse().expect("Not a number!");
    println!("The value of guess is {}", guess);

    // Scalar Types
    // floating point types
    let _x = 2.0; // f64 (default)

    let _three_point_zero: f32 = 3.0; // f32 

    // Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("these are the values for addition, subtraction, multiplication, division, remainder, {} {} {} {} {}", sum, difference, product, quotient, remainder);

    // The boolean type
    let _t = true;

    let _f: bool = false; // with explicity type annotation

    // The character type
    let _c = 'z';
    let _z = 'Ƶ';
    let _heart_eyed_cat = '😻';

    // Compound Types 

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tup;

    println!("The value of _y is: {}", _y);

    let _five_hundred = tup.0;

    let _six_point_four =  tup.1;

    let _one = tup.2;

    // The Array Type
    let a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let _a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    // Invalid Array Element Access
    // let element = a[10] -> compiles but exits with an error when it runs 


    println!("// Functions");

    println!("Hello, world!");

    another_function();



}


fn another_function() {
    println!("Another function.");
}
