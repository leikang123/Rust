
use rand :: prelude ::*;
fn main() {
    // 定义一个伪随机生成器
    let mut x = thread_rng();
    println!("{}",x.gen_range(0,20));
    print!("{}",x.gen::<f64>());
    println!("{}",if x.gen() {"Heads"} else {" Tails"});

}