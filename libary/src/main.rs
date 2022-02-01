
fn main() {
    by_moving();
    by_clone();
    by_muting();
    println!("Hello, world!,beijing!!!");
}
fn by_moving(){
    let x = "hello".to_string();
    println!("x:{:?}",x);
    let y = "world!";

    let z = x + y;
    println!("z={:?}",z);
    let k = String::from("shanghai hello");
    let m = String::from("xuhuiqu");
    println!("j:{:?}",k+&m);
}
fn by_clone(){
    let x = "leikang".to_string();
    let  y = &x;
    println!("x:{},y:{}",x,y);
    let x2 = 9;
    let y2=x2;
    let y5 = &x2;
    println!("x2:{},y2:{}",x2,y2);
    let x3 = String::from("leikang is value");
    let y3 = &x3;
    let y4 = &x3;
    println!("x3:{},y3:{}",x3,y3);
}
fn by_muting(){
    let mut x = "leikang is ciru".to_string();
    let sdf = "usa";
    x.push_str(sdf);
    println!("{:?}",x);
    let mut x2 = String::from("lkjh");
    let y2 = "hgf";
    x2.push_str(y2);
    println!("x2:{}",x2);


    
}