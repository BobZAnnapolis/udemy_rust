fn main() {
    println!("");
    // single comments in Rust use 2 backslashes
    /* 
    ** multi-line comments use this format
     */

     // println! is used to print lines to stdout
     // ! means to add a NEWLINE - it's a built-in Rust macro
    println!("Hello, world - using cargo new, create & run!");
    println!("");

    // learning some basic output commands
    // to print an integer, need a placeholder {}
    println!("Printing a constant : {}", 10);
    println!("");

    // to print strings, use placeholders, supply vals at end
    println!("Strings : My 1st name is {}, my lasy name is {}.", "Aubrey", "Kate");
    println!("");
    
    // printing on same line
    print!("Printing on 1 line w/o 'ln'");
    print!(" : Same line as well here");
    println!("");
    println!("");

    // printing escape codes
    println!("\n\n\nUsing 3 \\n's to skip 3 lines from above");
    println!("Printing using a \\t \t tab code");
    println!("");

    // next example uses \\r to overwrite text
    println!("Placeholder : This text will be overwritten by \\r on next line");
    println!("Placeholder : This text will be overwritten by \rOverwriting some of the text using \\r");
    println!("");

    // positional arguments
    println!("\npositional args inside brackets\ni am doing {{2}}, from {{1}} years and i {{0}} -> like, 20, programming");
    println!("\ni am doing {2}, from {1} years and i {0}", "like", 20, "programming");
    println!("");
    
    // named arguments
    println!("\nnamed args {{language}} is a system programming language which is good to {{activity}} in -> language=rust, activity=code");
    println!("\n{language} is a system programming language which is good to {activity} in", language="Rust", activity="code");
    println!("");    

    // learning how to print basic math
    println!("The summation of 25 + 10 = {{}}, 25+10");
    println!("The summation of 25 + 10 = {}", 25+10);
    println!("");

    // plenty of other examples
    println!("\nPlenty of other examples for binaries, octets, etc \n");
    
}