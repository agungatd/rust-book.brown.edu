fn main() {
    fun1();
}

fn fun1() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let num: &i32 = &v[2];

    println!("Third element is {}", *num);
    println!("Again, the third element is {}", *num);
    println!("Check again, 3rd element is {}", *num);
    
    v.push(4);
}
