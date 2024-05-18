fn main() {
    
    println!("// Functions");

    println!("Hello, world!");

    another_function(5, 6);

    println!("// Function Parameters");

    println!("// Statements and Expressions in Function Bodies");

    let x = 5; // This is a statement 

    //  let x = (let y = 6); // this will fali because statements dont return anything and cant be assigned as values

    let y = {
        let x = 3;
        x + 1
    }; // this is valid mainly because the line above is an expression as it doesn't end with a ; so it returns the value 4 which is then assigned to y 

    println!("The value of y is: {}", y);

    println!("// Functions with Return Values");
    let x = five();

    println!("The value of x is: {}", x);
    
    let y = plus_one(8);

    println!("The value of y is: {}", y);

    println!(" // Control Flow");

    println!("// if Expressions");


    
}


fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}


