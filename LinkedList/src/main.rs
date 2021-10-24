
use std::cell::RefCell;

   use std::rc::Rc;
  #[derive(Clone)]
   struct Node {
       value: i32,
       next: Option<Rc<RefCell<Node>>>
   }

   struct TransactionLog {
       head: Option<Rc<RefCell<Node>>>,
       tail: Option<Rc<RefCell<Node>>>,
       pub length: u64
}
main(){

}