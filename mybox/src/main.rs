use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // hello(m.deref())
}
