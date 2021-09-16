struct Student{
    name:String,
    age:i32,
    score:i32,
}
impl Student{
    // 静态方法或者非静态方法
    fn static_method(w1:String,w2:i32,w3:i32) -> Student{
        Student{
            name:w1,
            age:w2,
            score:w3,
        }

    }
    // 显示方
    fn dis_p(&self){
    format!("{} {} {}",self.name,self.age,self.score);
    }
}

fn main() {
    let x = Student::static_method("java".to_string(),23,87);
      x.dis_p();
}
