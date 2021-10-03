fn factorial(num: u64) -> u64 {
  match num {
      _ => factorial(num - 1) * num,
  }
}

fn main() {
  println!("{} ", factorial(4));
}