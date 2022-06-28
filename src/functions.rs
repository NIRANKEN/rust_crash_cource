pub fn run() {
  greeting("Hello", "Bob");

  // let get_sum = add(5, 6);
  // println!("Sum: {}", get_sum);
  println!("Sum: {}", add(5, 6));

  // Closureというらしい
  let add_nums = |n1: i32, n2: i32| n1 + n2;
  println!("C Sum: {}", add_nums(3, 4));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}
