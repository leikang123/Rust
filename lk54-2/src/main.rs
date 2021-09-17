fn main() {
    let check = my_all(6);
    if check.is_ok(){
        println!("3 is ok")
    }
    let check2 = my_all(2);
    if check2.is_err(){
        println!("Not is ")
    }

}
fn my_all(a:i32) -> Result<String,String>{
    if a % 3 == 0{
        Ok("java is ".to_string())
    }else {
        Err("Rust is ".to_string())
    }
}
