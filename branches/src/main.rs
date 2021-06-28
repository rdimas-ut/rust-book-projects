fn main() {
    let number  = 3;

    if number < 5 {
        println!("condition was true");

    } else {
        println!("condition was false");
    }

    // if conditions only take a boolean value
    // unlike C++, Ruby, or JavaScript, Rust 
    // doesn't try to evalute non-Boolean types
    // to boolean
    
    if number != 0 {
        println!("number was something ther than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // Types must be the same
    let number = if condition {5} else {6};

    println!("The value of number is: {}", number);

    
}
