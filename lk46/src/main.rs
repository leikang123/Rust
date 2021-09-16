enum Student {
    good,
    much
    
}
fn pri_stu(w:Student){
    match w {
        Student::good =>{
            println!("好的汗")
        },
        Student::much =>{
            println!("乖的")
        }
    }
}
fn main() {
    let x_y = Student::good;
    let z_x  = Student::much;
    pri_stu(x_y);
    pri_stu(z_x);
}
