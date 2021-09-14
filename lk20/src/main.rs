fn main() {
let x = String::new();
let y_x = x.to_string();
println!("x={}",x);
println!("y_x={}",y_x);
println!("y_x len is {}",y_x.len());
println!("x is capacity {},y_x {}",x.capacity(),y_x.capacity());

let a = "Rust is Fun";
let b = &a;
let c_c = b.to_string();
println!("a={}",a);
println!("b={}",b);
println!("c_c={}",c_c);
println!("a.len is {}",a.len());
println!("b.len is {}",b.len());
println!("c_c.len is {}",c_c.len());
// println!("a is capacity {}",a.capacity());

let d = String::from("wangwang");
let e_e = d.to_string();
let f = &e_e;
println!("d={}",d);
println!("d is capacity {}",d.capacity());
println!("d is capacity {}",d.capacity());
println!("d is len {}",d.len());
println!("e_e={}",e_e);
println!("f={},f is len :{}",f,f.len());

}
