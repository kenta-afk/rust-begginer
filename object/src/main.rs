pub trait Draw {
    fn draw(&self) {}
}

pub struct A(usize);
pub struct B(isize);


impl Draw for A {}
impl Draw for B {}

pub struct Screen1 {
    pub components: Vec<Box<dyn Draw>>,
}

pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

fn main() {
    // Using Box<dyn Draw> to store different types
    let screen1 = Screen1 {
        components: vec![Box::new(A(1)), Box::new(B(2))],
    };

    // Using generics to store the same type
    let Screen2 = Screen2 {
        components: vec![A(1), B(2)],
    };
}
