fn main() {
    // 型を合わせる
    let x: usize = 5;
    let y: f64 = 5.0;
    let z: f64 = (x as f64) / y;
    println!("z: {}", z);

    for elm: i32 in 0..5 {
        println!("{}", elm)
    }
}
