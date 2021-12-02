#![allow(dead_code)] 
pub struct Course{
   name: String,
   id : i32,
}
fn main(){
  let c1:&Course;
    {
    let c2: Course = Course {
      name : String::from("Rust"),
      id:101,
    };
    print!("c2 {}",c2.name);
    }
    // c1 = &c2; // allocated reference to a memory location that is dropped
}