#![allow(dead_code)]
#[derive(Debug)]
// declare an enum
enum TrafficSignal{
  Red, Green, Yellow
}
//implement a Traffic Signal methods
impl TrafficSignal{
  // if the signal is red then return
   fn is_stop(&self)->bool{
     match self{
       TrafficSignal::Red=>return true,
       _=>return false
     }
   }
}
fn main(){
  // define an enum instance
  let action = TrafficSignal::Red;
  //print the value of action
  println!("What is the signal value? - {:?}", action);
  //invoke the enum method 'is_stop' and print the value
  println!("Do we have to stop at signal? - {}", action.is_stop());
}