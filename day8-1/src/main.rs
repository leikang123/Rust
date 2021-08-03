fn main() {
    // 定义一个字符串
    let x = String::from("helllo");
    // 定义y，将x的值复制给y
    let y =x.clone();
    println!("x = {},y = {}",x,y);
    // 定义一个方法，将x的值传给方法
    add_lk(x);

    // 定义一个数字变量a
    let a = 8;
    // 定义一个数字变量b,将a的值传给b
    let b = a;
    println!("a = {},b = {}",a,b);
    // 定义一个方法，将x的值传给方法
    add_int(a);

    println!("Hello, world!");
}
// 括号内的第一个是带参数的变量，第二个参数是该变量的类型，字符串类型为String
fn add_lk(some_string:String){
    println!("some_string : {}",some_string);
}
// 该参数类型为数字类型,i32/64都可以
fn add_int(some_interget:i64){
    println!("some_integert : {}",some_interget)
}