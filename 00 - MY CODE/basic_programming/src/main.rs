fn main(){
    // this is our first program in this course
     // this is the second line of commen

    /* 
     * This is a multiline comment.
     * */

    // printLN >> print a line, with endline character.
    // ! >> rust macro. This is substituted by other code by the compiler
    println!("Hello from rust");

    //print/*ln*/!("Hello from rust");

    // PRINTING STRINGS AND NUMBERS

    // this does not work :
    //print!(10);

    // this does work:
    println!("The value is {}", 10);

    println!("My first name is {} and my last name is {}", "John", "Deer");

    // text on the same line:
    print!("This is a print command");
    print!("This goes on the same line");

    print!("This is going to be
    printed on multiple
    lines");


    // Escape sequences
    // conatins \ with a combination of other characters
    print!("\n >new\nline");
    print!("\t >inserted tab \n");
    print!("This text will be overwritten\r >carriage return. Move completely to the right");

    println!("\\n\n the first slash n is going to be printed");

    println!("This will print a single quote \' and this will print double quotes\"");

    // positional placeholders
    println!("3th argument: {2}, first arg: {0}, arg #2: {1}. The first one again:{0}", "A", "B", "C");
    // named placeholders
    println!("dog: {breed}, human: {homan_name}", breed="Golden Retriever", homan_name="Sara");

}




