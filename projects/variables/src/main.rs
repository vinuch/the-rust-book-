fn main() {
    let mut x = 4;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing example 
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);    

    // type can be changed when shadowing 
    let spaces = "    ";
    let spaces = spaces.len();
}
