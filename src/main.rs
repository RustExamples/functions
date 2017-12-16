// Entire function definition is a statement
fn main(){
    // Statements perform action and return no value
    // "let y = 6" or "let y = 5 + 6" as whole is a statement
    // while "6" or "5 + 6" is an expression
    let y = 6;
    let y = 5 + 6;

    // Blocks (everything within {}) that create new scope, is an expression
    let y = {
        let x = 3;
        // No ";" at end denotes that its an expression
        // If ";" added then it becomes a statement
        x + 1
    };

    println!("y is {}", y);
}