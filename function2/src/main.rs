use std::panic;
use std ::panic::AssertUnwindSafe;

fn main() {
let mut x : Vec<i32> = vec![1]; 
let mut y : Vec<i32> = vec![2]; 
panic ::catch_unwind(AssertUnwindSafe(|| {
x.push( 10);
panic! ("user paswnic");
 y.push(100);
} ) ).ok();
println! ("Observe corruptted data.{:?}{:?}", x, y) ;
}
