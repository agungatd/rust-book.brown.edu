
fn main() {
    // variable and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3 hours is {THREE_HOURS_IN_SECONDS} seconds");

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /* this will compile due to shadowing (using let) */
    // let spaces = "   ";
    // let spaces = spaces.len();
    /* while this will NOT compile */
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
