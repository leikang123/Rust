#[derive(Debug)]
enum Color{
    Red(String),
    Yellow(String),
    Blue(String)
}
fn main() {
    let mut x = Color::Blue(String::from("bulu"));
    println!("{:?}",x);
}
