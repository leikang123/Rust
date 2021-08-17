// 结构体名称
struct Person {
    // 属性
    name: String,
    age: u8,
}
// 实现结构体Descriptive 规定了实现者必需有 describe(&self) -> String 方法。
// 我们用它实现一个结构体：
// impl <特性名> for <所实现的类型名>
impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}
trait Descriptive {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}

struct Person {
    name: String,
    age: u8,
}
// 案例
/*impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

fn main() {
    let cali = Person {
        name: String::from("Cali"),
        age: 24
    };
    println!("{}", cali.describe());
}*/
