extern crate lkrust;
use lkrust::{Ash,run_simulation};
// #[allow(dead_code)]
fn main(){
    let mut  fern =Ash{
        size:1.0,
        growth_rate:0.001
    };
    run_simulation(& mut fern,1000);
    println!("final fern size:{}",fern.size)
}
#[test]
fn math_works(){
    let x =1;
    // assert!(x.is_positive());
    assert_eq!(x+1,2);

}