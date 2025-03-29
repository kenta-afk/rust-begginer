use std::cmp::PartialOrd;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn main() {
    let numbers_list = vec![1, 2, 3, 4, 5];

    println!("The largest number is: {}", largest(&numbers_list));

    let numbers_list = vec![100, 2000, 2220, 3000, 4000];

    println!("The largest number is: {}", largest(&numbers_list));

    let numbers_list = vec![1.0, 2.0, 3.0, 4.21, 5.11];

    println!("The largest number is: {}", largest(&numbers_list));
}
