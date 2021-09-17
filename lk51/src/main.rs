


fn main() {
    let my_val: Option<&str> = Some("Rust Programming!");
    print(my_val); // invoke the function
   
}
fn print(my_val: Option<&str>){
     if my_val.is_some(){ // check if the value is equal to some value
        println!("my_val is equal to some value");
    }
    else{
        println!("my_val is equal to none");
    }
}