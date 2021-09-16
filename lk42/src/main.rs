// 定义元祖结构，元祖结构是没有大括号体的，只有括号以及参数
struct Student(String,i32,i32);
fn main() {
    //  创建实力
    let x_s = Student("delang".to_string(),12,76);
        println!("x_s name {},age {} ,id {}",x_s.0,x_s.1,x_s.2);
    let x_s2 = Student("java ".to_string(),23,89);
    println!(" x_s2 name {},age {} ,id {}",x_s2.0,x_s2.1,x_s2.2);
    let x_s3 = Student("jhgshg".to_string(),87,11);
    println!("x_s3 name {},age {},id {}",x_s3.0,x_s3.1,x_s3.2);
}
