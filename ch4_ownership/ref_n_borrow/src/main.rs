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

    // println!("Vector is now {:?}", v); cannot borrow `v` as immutable 
    //                                    because it is also borrowed as mutable

    // Mutable references can also be temporarily "downgraded" to read-only references
    let num2: &i32 = &*num;
    println!("*num={} *num2={}", *num, *num2); // 3 3

    *num += 1; // from 3 + 1 become 4 (*num)

    println!("Third element is {}", *num); // 4
    println!("Vector is now {:?}", v); // [1, 2, 4]
}
