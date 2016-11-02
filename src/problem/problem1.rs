use std::collections::LinkedList;

pub fn run_1() {
  let mut target = LinkedList::new();

  for i in 1..10 {
    target.push_back(i);
  }

  let res_for = sum_by_for(&mut target);
  let res_while = sum_by_while(&mut target);
  let res_rec = sum_by_recursive_fn(&mut target);

  println!("for:   {}", &res_for);
  println!("while: {}", &res_while);
  println!("rec:   {}", &res_rec);
}

fn sum_by_for(list: &mut LinkedList<i32>) -> i32 {
  let mut sum: i32 = 0;
  for i in list {
    sum = sum + *i;
  }
  return sum;
}

fn sum_by_while(list: &mut LinkedList<i32>) -> i32 {
  let mut list_ = list.clone();
  let mut sum = 0;
  while !list_.is_empty() {
    let head = list_.pop_front().unwrap();
    sum = sum + head;
  }
  return sum;
}

fn sum_by_recursive_fn(list: &mut LinkedList<i32>) -> i32 {
  if list.is_empty() {
    return 0;
  }

  let front = list.pop_front().unwrap();
  return front + sum_by_recursive_fn(list);
}