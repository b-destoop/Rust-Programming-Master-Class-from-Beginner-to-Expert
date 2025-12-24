fn main() {
    // variables are IMMUTABLE by default!!
    let x: f64 = 14.0;
    println!("Data type float: {}", x);


    let mut y: u32 = 10;
    println!("The mutable value of y is now {}", y);
    y = 11;
    println!("The mutable value of y is now {}", y);

    // ********************
    // Variables are either 
    //      - Scalar
    //         - Single value types (int, float, bool, char)
    //      - Compound
    //          - Multiple values combined?
    // ********************
    let maxval = std::u8::MAX;
    let minval = std::u8::MIN;
    println!("The maximum value of an unsigned int 8 is... {}", maxval);
    println!("The minimum value of an unsigned int 8 is... {}", minval);

    println!("Printing multiple values of undefined types can be done like so:{:?}", (x, y, maxval, minval, true));

}
