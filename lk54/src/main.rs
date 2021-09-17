fn main() {
    println!("{:?}",dis_v(6));
    println!("{:?}",dis_v(2));

}
fn dis_v(y:i32) -> Result<String,String>{
    if y %3 ==0 {
        Ok("keyide".to_string())
    }else {
        Err("bukenneg".to_string())
    }
}
