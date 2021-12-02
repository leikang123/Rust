fn main() {
    let mut q = 10;
    q+=12;
    let mut x = &q;
   // println!("x:{}",x);
    let a = & mut x;
    println!("a:{}",a);
    //let b= & mut a;
   // println!("b:{}",b);
    let mut e = 123;
    let d  = & mut e;
    let f =& d;
    let g = & f;
    let mut t =(23,90);
    let m = & mut t;
    let n = & mut m.0;
       *n =278;
       println!("n:{}",n);


    

}
