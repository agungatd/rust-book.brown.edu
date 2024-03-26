fn main() {
    /* If Expression */
    // if is an expression so can be used on the right side of let statement.
    let cond = true;
    let x = if cond { 1 } else { 0 };
    println!("{x}");
    
    // The condition to an if-expression must be a boolean. 
    // Rust does not have a concept of "truthy" or "falsy" values.
    // let cond = 1 
    // let x = if cond { 1 } else { 0 }; // this will NOT compile

    /* Loops */
    // loops has 3 expression loop, while, for   
    value_from_loop();
}

fn value_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {result}");
}

fn loop_label() {
    // counting_up is a label, when break happen in inner loop
    // it will break outo the counting_up loop not the inner loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // reverse the range with method rev
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}