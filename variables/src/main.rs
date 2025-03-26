fn main() {
    // 再代入可能な変数
    let mut x: i32 = 5;
    //定数
    const CONSTANT: usize = 100;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    let y: i32 = 5;

    let y: i32 = y + 1;

    // shadowing
    {
        let z: i32 = 5;
        let y: i32 = y * 2;
        println!("The value of y in the inner scope is: {}", y);
        println!("The value of z is: {}", z);
    }

    // println!("The value of z is: {}", z); // error: cannot find value `z` in this scope

    println!("The value of y is: {}", y);


    let some_strings: &str = "aaa";
    println!("The value of some_strings is: {}", some_strings);

    let some_strings: usize = some_strings.len();
    println!("The value of some_strings is: {}", some_strings);
}
