struct Student{
    name:String,
    age:i64,
    fale:String,
    score:i64,
}
fn student_stu(w1:Student,w2:Student) -> Student{
    if w1.name == "java"{
    return w1;
    } else {
        return w2;
    }
    println!("start");

}
fn main() {
    //创建实力
    let x_1 = Student{
        name:String::from("java"),
        age:13,
        fale:String::from("men"),
        score:87,
    };
    let x_2 = Student{
        name:String::from("rust"),
        age:18,
        fale:String::from("women"),
        score:34,
    };
    //定义变量，将结构体的实力传给次变量并打印
    let y_student = student_stu(x_1,x_2);
    println!("name {},age {},fale {},score {},",y_student.name,y_student.age,y_student.fale,y_student.score);
}
