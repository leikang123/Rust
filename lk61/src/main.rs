#![allow(dead_code)] 
struct Course{
   name: String,
   id : i32,
}
fn get_course<'a> (c1: &'a Course, c2: &'a Course) -> &'a Course {
  if c1.name == "Rust" {
     c1
  }
  else {
     c2
  }
}
fn main(){
  let c1: Course = Course {
      name : String::from("Rust"),
      id:101,
    };
    
   let c2: Course = Course {
      name : String::from("C++"),
      id:101,
    };
    get_course(&c1, &c2);   
}