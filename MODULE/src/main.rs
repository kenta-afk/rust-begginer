pub mod second;
pub mod third;


use second::hello;
use third::return_three::return_three;

fn main() {
    println!("Hello, world!");
    hello();
    let value = return_three();
    println!("The value is: {}", value);
}
