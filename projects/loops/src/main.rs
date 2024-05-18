fn main() {
    loop {
        println!("again!");
        break;
    }

    println!("// Conditional loops with while");
    
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
    
    println!("// Looping Through a Collection with while");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }


    println!("// Looping Through a collection with for");

    let b = [10, 20, 30, 40, 50];

    for element in b.iter() {
        println!("the value is: {}", element);
    }


    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!!");
}
