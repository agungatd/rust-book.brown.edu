fn main() {
    /* There are four types of scalar types */
    // integer, float, boolean, and char

    // If x : u8 = 0, what will happen when computing x - 1?
    // This expression will panic in debug mode and return 255 in release mode.
    // due to the integer overflow

    // char use single quote '' oppose to string literal ""
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    /* There are two primitive compound types (tuple and array) */
    // Tuple
    // tuple group variety of types
    // tuple has fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // tuple destructuring
    let (x, y, z) = tup;

    // accessing tuple using period (.)
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Array
    // group same type of elements
    // in rust array has fixed length
    // there is Vector type that can change size, use this if unsure
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // array with the same value
    let a = [6; 3]; // [6,6,6] 😈
}
