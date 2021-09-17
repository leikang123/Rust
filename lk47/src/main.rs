// make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
//define an enum
enum KnightMove{
   Horizontal, Vertical
}
#[derive(Debug)]
// make this `struct` print values of type `enum`  with `fmt::Debug`.
struct Player {
   color:String,
   knight:KnightMove
}
fn main() {
      // instance 1
      let p1 = Player{
      color:String::from("black"),
      knight:KnightMove::Horizontal
   };
      // instance 2
      let p2 = Player{
      color:String::from("white"),
      knight:KnightMove::Vertical
   };
   println!("{:?}", p1);
   println!("{:?}", p2);
}