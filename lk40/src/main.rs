pub struct Student{
    name:String,
    fale:String,
    age:i32,
    id:i32,
}
// 实现结构体，并定义方法实现功能
impl Student{
    fn stu_met( &self ) {
        format!("{} {}",self.name,self.fale); 

    }
}

fn main() {
    let stu_x = Student {
        name:String::from("leikang"),
        fale:String::from("men"),
        id :22,
        age:19,

    };
    println!("name {},fale {}",stu_x.name,stu_x.fale);
}
