fn main() {
    // define a variable
    let str = String :: from("Educative");
    // define the index value to be found
    let index = 12;
    lookup(str, index);
  }
  fn lookup(str: String, index: usize) {
    let matched_index = match str.chars().nth(index){
      // execute if match found print the value at specified index 
       Some(c)=>c.to_string(),
       // execute if value not found
       None=>"No character at given index".to_string()
       };  
    println!("{}", matched_index);
  }