#[derive(Debug)]
enum  Student {
    name(String),
    age(i32)
    
}

fn main() {
    // 初始化枚举类
    let x_y = Student::name("leikang".to_string());
    let y_z = Student::age(22);
    println!("name {:?},age {:?}",x_y,y_z);
}
