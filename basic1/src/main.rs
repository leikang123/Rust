// 结构体代码实现
struct Student{
    name: &'static str,
    socre :i32
}
struct Color(i32,i32,i32);
fn main() {
    let b_b = Color(23,45,78);
let socre =43;
let name = "lk";

// 结构体实力话
let mut s  = Student{
    name:"iou",
    socre:98
};
// s.socre = 98;
let s1 = s.name;
let s2 = s.socre;
 println!("name:{},socre:{}",s1,s2)
}
