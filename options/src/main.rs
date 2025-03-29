//標準である
//値が存在しない可能性がある場合,nullを表現する代わりに使う
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
   let mut maybe_number: Option<i32> = Some(5);
   println!("{:?}", maybe_number);
   let maybe_number: Option<i32> = None;
   println!("{:?}", maybe_number);
}
