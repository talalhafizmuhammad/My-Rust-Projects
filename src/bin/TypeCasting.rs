fn main() {
    // Integer to integer (safe casts with `as`)
    let a: i32 = 100;
    let b: u32 = a as u32;   // i32 to u32 (possible truncation if negative)
    let c: i64 = a as i64;   // i32 to i64 (safe widening)
    println!("a={}, b={}, c={}", a, b, c);

    // Integer to float
    let f1: f32 = a as f32;
    let f2: f64 = a as f64;
    println!("f1={}, f2={}", f1, f2);

    // Float to integer (truncates decimal part)
    let x: f64 = 123.456;
    let y: i32 = x as i32;
    println!("x={}, y={}", x, y);

    // Unsigned to signed and vice versa
    let u: u8 = 255;
    let i: i8 = u as i8; // overflow wrap-around, 255 becomes -1
    println!("u={}, i={}", u, i);

    // Char to integer and back
    let ch: char = 'A';
    let ch_code: u32 = ch as u32;  // Unicode scalar value
    let ch2 = std::char::from_u32(ch_code).unwrap();
    println!("ch={}, code={}, ch2={}", ch, ch_code, ch2);

    // Boolean to integer (true = 1, false = 0)
    let t: bool = true;
    let f: bool = false;
    let t_int = t as u8;
    let f_int = f as u8;
    println!("true={}, false={}", t_int, f_int);

    // Integer to boolean (NO direct cast allowed)
    // let b1: bool = 1 as bool; // ERROR: casting integer as bool not allowed

    // Using `From` and `TryFrom` traits for conversions
    use std::convert::{TryFrom, TryInto};

    let num_i8: i8 = 100;
    let num_u8: u8 = u8::try_from(num_i8).unwrap();  // Converts i8 to u8 if positive
    println!("num_i8={}, num_u8={}", num_i8, num_u8);

    // This will fail because -10 is negative and u8 can't represent it
    let neg_i8: i8 = -10;
    let neg_to_u8 = u8::try_from(neg_i8);
    match neg_to_u8 {
        Ok(val) => println!("Converted: {}", val),
        Err(e) => println!("Conversion failed: {:?}", e),
    }

    // Example: string to integer parsing
    let str_num = "123";
    let parsed_num: i32 = str_num.parse().unwrap();
    println!("Parsed number: {}", parsed_num);

    // String to integer parsing failure handling
    let bad_str = "abc";
    let parsed_bad: Result<i32, _> = bad_str.parse();
    match parsed_bad {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Failed to parse: {}", e),
    }
}
