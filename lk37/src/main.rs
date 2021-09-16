struct Student{
    name:String,
    age:i64,
    fale:String,
    socre:i64,
}
fn main() {
    // 定义变量
    let mut x_student = Student{
        name:String::from("leikang"),
        age:12,
        fale:String::from("men"),
        socre:98

    };
 
    println!("name: {},age {},fale {},socre {}",x_student.name,x_student.age,x_student.fale,x_student.socre);
     x_student.name ="wangwanh".to_string();
     x_student.socre=19;
     println!("name {},socre {}",x_student.name,x_student.socre)
}

