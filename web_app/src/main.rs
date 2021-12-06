use rand::prelude::*;
fn gen_fla(gentor:&mut ThreadRng) -> f64{
    let placeholder:f64 = gentor.gen();
    return placeholder * 10.0
}
fn main() {
    let mut rng = rang:: thread_rng();
    
    println!("Hello, world!");
}
