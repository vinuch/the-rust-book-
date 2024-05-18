fn main() {
    let number = 8;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Handling Multiple Conditions with else if 

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisble by 2");
    } else {
        println!("number isnot divisible by 4, 3, or 2");
    }

    // Using if in a let Statement 
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };    

    println!("The value of number is: {}", number);

}


