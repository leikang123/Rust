 pub struct Ash{
    pub size :f64,
    pub growth_rate:f64
}

 impl Ash{
   pub fn grow(& mut self){
        self.size *=1.0 + self.growth_rate;
    }
}
pub fn run_simulation(fern:& mut Ash,days:usize){
    for _ in 0 .. days {
        fern.grow();

    }
}

fn main() {
    let mut fern = Ash{
        size : 1.0,
        growth_rate:0.001
    };
    run_simulation(&mut fern,1000);
    println!("final fern size :{}",fern.size);
}
