
// trait类型
trait doubt {
    fn dou(&self) -> Self;
}

impl doubt for i32 {
    fn dou(&self) -> Self {
        *self * 2
    }
}

fn main() {
    let x: i32 = 10.dou();
    print!("{}", x)
}
