fn main() {
    // VARIABLES
    let a = 5; // immutable
    a = 10; // >> error
    let a = 5.5; // a on line 6 SHADOWS a on line 4

    let mut b = 10; // mutable variable
    b = 5; // >> no error
    b = 5.5; // >> error. Type of i32 will be infered by the compiler on line 8

    let c: i32 = 1; // types can be declared explicitely

    let d = a + b; // >> error. a and b are of incompatible types (float and int)
    let d = a + b as f32; // >> no error bc type casting

    
    // SHADOWING
    // > redeclaration and redefinition of variables after they have been declared already
    // shadowing of same (immutable) type
    let e = 3;
    let e = 5;

    // shadowing of different type
    let g: i32 = 2;
    let g: char = 'A'; // >> this g 'shadows' the previous g

    // shadowing in code segments ( {code} )
    let f = 10;
    {
        let f = 12;
        println!("f in segment: {}", f); 
        // > f in segment : 12
    }
    println!("f outside of segment: {}", f);
    // > f outside of segment: 10

    // BUT!!!!
    let f = 10;
    {
        f = 12;
        println!("f in segment: {}", f); 
        // > f in segment : 12
    }
    println!("f outside of segment: {}", f);
    // > f outside of segment: 12
    

    // CONSTANTS
    const MAX_SALARY: i32 = 100_000;

}
