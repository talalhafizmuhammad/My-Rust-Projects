fn main() {
    // 1. Mismatched types
    let x: i32 = 10;
    let y: u64 = x;  // Error: expected u64, found i32

    // 2. Cannot add different numeric types
    let a: i32 = 5;
    let b: f32 = 2.0;
    let c = a + b; // Error: no implementation for `i32 + f32`

    // 3. Using string instead of number
    let s = "100";
    let n: i32 = s; // Error: expected i32, found &str

    // 4. Out-of-range literal
    let big_num: u8 = 300; // Error: literal out of range for `u8`

    // 5. Overflow on compile-time constants (in debug mode panics)
    let overflow = 255u8 + 1; // Runtime panic on overflow

    // 6. Invalid casts
    let num = 10;
    let casted = num as bool; // Error: cast to bool not allowed

    // 7. Borrow checker conflicts
    let mut s = String::from("hello");
    let r1 = &s;          // immutable borrow
    let r2 = &mut s;    // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}, {}", r1, r2);

    // 8. Trying to index a string with usize directly (needs slicing)
    let my_string = String::from("Rust");
    let c = my_string[0]; // Error: cannot index into a String directly

    // 9. Using a variable before initializing
    let x: i32;
    println!("{}", x); // Error: use of possibly uninitialized variable `x`

    // 10. Function expects different types
    fn takes_i32(n: i32) {
        println!("Number: {}", n);
    }
    takes_i32("123"); // Error: expected i32, found &str

    // 11. Pattern matching with incompatible types
    let val = 10;
    match val {
        "ten" => println!("Ten"), // Error: expected integer pattern, found &str
        _ => println!("Other"),
    }

    // 12. Returning wrong type from function
    fn foo() -> i32 {
        "hello" // Error: expected i32, found &str
    }

    println!("All errors above will prevent compilation.");
}
