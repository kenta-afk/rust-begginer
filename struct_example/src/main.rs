#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//特定の構造体に対してメソッドを定義する
impl Rectangle {
    //正方形の構造体を返す。SelfはRectangleを指す
    fn square(width: u32) -> Self {
        Self {
            width,
            height: width,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {

    let mut rect: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let square = Rectangle::square(60);

    rect.width = 60;

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!(
        "The area of the square is {} square pixels.",
        square.area()
    );

    

    println!("rect :{:?}", rect);
    println!("square: {:?}", square);

    rect.set_width(100);

    println!("rect :{:?}", rect);
}

