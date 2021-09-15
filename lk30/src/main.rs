fn main() {
   let mut my_vec = Vec::new();
   println!("Empty Vector : {:?}", my_vec);
   my_vec.push(1);
   my_vec.push(2);
   my_vec.push(3);
   println!("Pushed elements 1 , 2 , 3 : {:?}", my_vec);
   my_vec.pop();
   println!("Popped value: {}", 3);
   println!("Popped element at last index : {:?}", my_vec);
   my_vec.remove(1);
   println!("Removed value: {}", 2);
   println!("Removed element at index 1 : {:?}", my_vec);
   println!("Size of vector is :{}", my_vec.len());
   println!("Does my vector contains 1 : {}", my_vec.contains(&1));
}
