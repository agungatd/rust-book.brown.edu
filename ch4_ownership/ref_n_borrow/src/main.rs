fn main() {
    // fun1();
    fun2();
}

fn fun1() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let num: &i32 = &v[2];

    println!("Third element is {}", *num);
    println!("Again, the third element is {}", *num);
    println!("Check again, 3rd element is {}", *num);
    
    v.push(4);
}

fn fun2() {
    // Mutable references provide unique and non-owning access to data.
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}
