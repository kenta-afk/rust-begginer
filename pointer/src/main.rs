fn main() {
    let a = vec![1, 2, 3];
    let barrowed_a = &a;
    let b = vec![1, 2, 3];
    // 参照を比較できない
    println!("equal? {}", *barrowed_a == b);
    println!("a: {:?}, b: {:?}", a, b);

    let mut moved_a = a; //move
    let muuttably_barrowed_a = &mut moved_a;
    //可変参照も同じ
    *muuttably_barrowed_a = vec![1, 2, 3, 4];
    println!("moved_a: {:?}", moved_a);
}
