fn main() {
    // function could have parameters
    // each parameter MUST have type annotation
    // argument is the concrete value provided into the parameter

    // statement: instruction that perform action and NOT return value.
    // expression: evaluate to resultant value.
    let y = 6; // statement
    // expression can be part of statement e.g 1 + 1
    let x = 1 + 1;
    // Expressions do not include ending semicolons
    let y = {
        let x = 3;
        x + 1 // if added ; at the end it'll become statement.
    };

    println!("The value of y is: {y}");

    // function return value must declare its return type.
    let a_hundred = plus_one(99);

    println!("a hundred: {a_hundred}");
}

fn plus_one(x: u32) -> u32 {
    x + 1
}