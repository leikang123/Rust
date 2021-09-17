fn main() {
    println!("{:?}",file_found(true));
    println!("{:?}",file_found(false));
}
fn file_found(x:bool) -> Result<i32,bool>{
    if x {
        Ok(100)
    }else{
        Err(false)
    }

}
