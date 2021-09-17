struct Student{
    name:String,
    code:i32,
    dept:Option<String>
}
fn main() {
     let cou = Student {
        name:String::from("rust"),
        code:120,
        dept:Some(String::from("tech"))
    };
    let cou2 = Student{
        name:String::from("java"),
        code:200,
        dept:None
    };
    println!("cou name {},id {},dept {}",cou.name,cou.code,cou.dept.unwrap_or("level".to_string()));
    println!("cou2 name {},code {},dept {}",cou2.name,cou2.dept.unwrap_or("No level".to_string()),cou2.code);
}
