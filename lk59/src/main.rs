/*#[allow(dead_code)]
fn main() {
    
    let str_1 = move_return_value_str_1();  // gives_ownership to str_1                                  
    
    println!("The function gives ownership to string by returning a value \nstring 1 :{}",str_1); // print value of str_1

    let str_2 = String::from("Rust Language");     // assigns a string object to str_2
    
    println!("This is a string declared \nstring 2 :{}",str_2); // print value of str_2   

    let str_3 = moves_str_2_return_str_2(str_2);  // str_2 is moved into the function argument
                                            // return value moves to str_3 
    println!("string 2 passes to the function and returns its value to string 3 \nstring 3 :{}",str_3); // print value of str_3                         
                                     
} // Here, str_3,str_2,str_1 goes out of scope respectively
 // str_3 dropped
 // str_2 moved
 // str_1 dropped
fn move_return_value_str_1() -> String {     // gives ownership 
                                             // value goes to that calls the function
    let my_string = String::from("Rust"); // my_string comes into scope

    my_string                              // my_string is returned 
}

fn moves_str_2_return_str_2(my_string: String) -> String { // my_string comes into 
                                                      // scope
    my_string  // my_string is returned 
}*/
fn main() {

    let a = String::from("Rust"); //variable a
  
    println!("This is a variable a: {}", a);
  
    let b = a; // moves value of a to b
  
    println!("Value of variable a is moved to b.\n b : {}", b);
    println!("Now a becomes invalid.Accessing a will give error");
    
   // let c = &a;
    //println!("This is a variable c trying to access value a: {}", c);
  }