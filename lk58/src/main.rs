/*pub mod chapter {
    pub mod lesson { // mod level 1
        pub mod heading { // mod level 2
            pub fn illustration() {  // mod level 3
              println!("Hi, I'm a 3rd level nested function");
            }
        }
    }
}
fn main() {
    chapter::lesson::heading::illustration(); // call the function
}*/
pub mod l1{
    pub mod l2 {
        pub mod l3{
            pub mod l4 {
                pub fn x_y(){
                    println!("lk is dldj");
                }
            }
        }
    }
}
use l1::l2::l3::l4;
fn main(){
l4::x_y();
}