fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn main() {
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: i32| x + 1;
    println!("add_one_v1: {}", add_one_v1(5));
    println!("add_one_v2: {}", add_one_v2(5));
    println!("add_one_v3: {}", add_one_v3(5));

    // クロージャは、環境をキャプチャすることができる
    let x = 5;
    let equal_to_x = |z| z == x;

    let y = 5;
    println!("equal_to_x: {}", equal_to_x(y));
}   