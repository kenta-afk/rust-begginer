#[derive(Debug)]
struct SomeStruct<'a> {
    part: &'a str,
}

fn main() {
    
    let s;

     {
        let novel = String::from("novel");
        s = SomeStruct{ part: &novel};
        println!("{:?}", &s);
     }

    // println!("{:?}", &s); // ここでエラーになる
}
