fn main() {
    let x = 32;
    let y = 64;

    another_function(x, y);
}

// Rust design require type annotation for function parameters
// so that we need not specify type of variables to be passed
// as arguments.
fn another_function(x: i32, y: i32){
    println!("x is {}, y is {}", x, y);
}
