// Functions can return values
// "-> i32" denotes return type and name isn't required
fn add(a: i32, b: i32) -> i32 {
    // Last expression is implicitly returned
    // "return" can also be used to return
    // adding ";" would result in compile time error, because this returns an empty tuple
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

fn main(){
    let add = add(5, 6);
    let sub = sub(6, 5);

    println!("add is {}, sub is {}", add, sub);
}