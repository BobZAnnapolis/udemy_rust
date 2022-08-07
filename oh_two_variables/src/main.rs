fn main() {
    println!("\nVARIABLES\n");
    println!("");

    // variables section
    // variables allocate space for each var - depends on var type
    // data type determines amount of memory allocated & operations 
    //
    // let keyword defines a var
    println!("let x = 15; // implicit or\nlet x1:i32 = 20; // explicit type declaration");
    let x = 15;
    let x1:i32 = 20;
    println!("implicit {0}; explicit {1}\n", x, x1);

    // Rust compiler must know all data types
    // by default, all vars are immutable - ie. not modifiable
    // to allow a var to be modified, use 'mut' keyword
    println!("vars are immutable by default\nuse keyword mut to allow them to be changed\nlet mut x = 15;\nprintln!('X = {{}}',x);\nx = 20;\nprintln!('X now = {{}},x');");
    let mut x = 15;
    println!("\nX = {}", x);
    x = 20;
    println!("X now = {};", x);

    // var rules
    println!("\nvars can only be composed of letters, digits, underscores - MUST begin with alpha or _\n");

    // 2 types of data types
    // - scalar
    // - compound
    // 
    // scalar data types
    // - integer
    //   - signed, i8, i16, i32, i64
    //   - unsigned, u
    // -
    println!("The maximum number of an i8  is : {}", std::i8::MAX );
    println!("The maximum number of an i16 is : {}", std::i16::MAX );
    println!("The maximum number of an i32 is : {}", std::i32::MAX );
    println!("The maximum number of an i64 is : {}", std::i64::MAX );

    println!("\nThe maximum number of an u8  is : {}", std::u8::MAX );
    println!("The maximum number of an u16 is : {}", std::u16::MAX );
    println!("The maximum number of an u32 is : {}", std::u32::MAX );
    println!("The maximum number of an u64 is : {}", std::u64::MAX );

    println!("\nThe maximum number of an f32  is : {}", std::f32::MAX );
    
    println!("\nThe value of bool=false is : {}", false );

}
