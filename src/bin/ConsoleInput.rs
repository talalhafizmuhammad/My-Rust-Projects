use std::io;
fn main() {
    //String input
    let mut input_str = String::new();
    println!("Enter string: ");
    io::stdin().read_line(&mut input_str).expect("Wrong Input!");
    println!("String you entered: {}", input_str.trim());

    //Integer Input
    let mut input_int = String::new();
    println!("Enter integer number: ");
    io::stdin()
        .read_line(&mut input_int)
        .expect("Failed to read line");

    match input_int.trim().parse::<i32>() {
        Ok(num) => println!("You entered: {}", num),
        Err(_) => println!("You entered wrong number!"),
    }

    //Floating point number
    let mut input_float = String::new();
    println!("Enter float number: ");
    io::stdin()
        .read_line(&mut input_float)
        .expect("Failed to read line");

    match input_float.trim().parse::<f64>() {
        Ok(num) => println!("You entered: {}", num),
        Err(_) => println!("You entered wrong number!"),
    }

    //Boolean Value
    let mut input_bool = String::new();
    println!("Enter boolean value: ");
    io::stdin()
        .read_line(&mut input_bool)
        .expect("Failed to read line");

    match input_bool.trim().parse::<bool>() {
        Ok(num) => println!("You entered: {}", num),
        Err(_) => println!("You entered wrong number!"),
    }
    
    //input character
    let mut input_char = String::new();
    println!("Enter a single character:");

    io::stdin().read_line(&mut input_char).expect("Failed to read line");

    // Get the first char if there is one
    let ch = input_char.chars().next();

    match ch {
        Some(c) => println!("You entered: '{}'", c),
        None => println!("No input!"),
    }

}
