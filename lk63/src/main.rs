

/*pub mod chapter {
    pub mod lesson {
        pub fn summary(){
            println!("Summary"); 
        } 
        pub mod heading {  
            pub fn illustration() {  
              println!("Content visualization");
            }
        }
    }
}
use chapter::lesson::heading;
use chapter::lesson;
 
fn main() {
    lesson::summary();
    heading::illustration(); 
}*/
fn main() {
    let a = String::from("Rust");
    let b = a; 
    println!("a:{} , b:{}", a, b); 
}