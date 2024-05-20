fn main() {

    //Hello World
    println!("Hello, world!");

    //Initializing numbers
    let x = -5;
    let y: u32 = 1;
    let z: f32 = 1000.32;
    println!("x: {},y: {},z: {}",x,y,z);

    //Initializing Strings
    let str = String::from("Hello, Rust");
    println!("{}",str);

    //Conditional Statements
    let is_even= true;
    if is_even {
        println!("Number is even");
    }else if !is_even {
        println!("Number is odd");
    }

    //Loops
    for i in 0..11 {
        print!("{} ",i);
    }

    //Memory Management
}