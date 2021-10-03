use rand::prelude::*;
fn gen_fla(gentor:&mut ThreadRng) -> f64{
    let placeholder:f64 = gentor();
    return placeholder * 10.0
}
fn main() {
    println!("Hello, world!");
}
