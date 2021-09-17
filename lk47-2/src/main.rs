#[derive(Debug)]
enum Student{
    men,
    women
}
#[derive(Debug)]
struct Score {
    // 结构体属性
    name:String,
    // 属性类型调用枚举名
    fale:Student
}

fn main() {
    // 初始化
    let x_1 = Score{
        name:String::from("java".to_string()),
        fale:Student::men
    };
    let x_2 = Score{
        name:String::from("rust".to_string()),
        fale:Student::women
    };
    println!("x_1 {:?}",x_1);
    println!("x_2 {:?}",x_2)
}
