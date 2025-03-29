#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

fn main() {
    let red = Color::Red;
    let yellow = Color::Yellow;
    println!("{}", color_to_str(&red));
    println!("{}", color_to_str(&yellow));
    find_maybe_number(Some(5));
    find_maybe_number(None);
    let maybe_number: Option<u32> = Some(10);
    if let Some(number) = maybe_number {
        println!("The number is: {}", number);
    } else {
        println!("No number found");
    }
}

fn find_maybe_number(maybe_number: Option<i32>) {
    match maybe_number {
        Some(number) => println!("The number is: {}", number),
        None => println!("No number found"),
    }
}

fn color_to_str(color: &Color) -> &str {
    // Red #FF0000
    // Green #00FF00
    // Blue #0000FF
    // Yellow #FFFF00
    match color {
        Color::Red => "#FF0000",
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
        Color::Yellow => "#FFFF00",
    }
}
