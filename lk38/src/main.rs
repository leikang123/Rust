// 定义结构体函数
struct Zoo{
    name:String,
    age:i32,
    Size:i64,
    color:String
}
fn zoo_x_y(w:Zoo){
    println!("name {},age {},Size {},color {}",w.name,w.age,w.color,w.Size);
}
fn main() {
    // 创建实力
    let x = Zoo{
        name:String::from("element"),
        age:21,
        Size:65,
        color:String::from("red"),
    };
    zoo_x_y(x);
}
