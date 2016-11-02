use std::collections::LinkedList;

pub fn run_2() {
  let mut list1 = LinkedList::new();
  let mut list2 = LinkedList::new();

  for i in 1..4 {
    list1.push_back(i);
  }

  for i in 11..14 {
    list2.push_back(i);
  }

  let list3 = combine_lists(&list1, &list2);

  print!("list: ");
  for i in list3 {
    print!("{}, ", i);
  }
  println!("");
}

fn combine_lists(list1: &LinkedList<i32>, list2: &LinkedList<i32>) -> LinkedList<i32> {
  let mut result_list = LinkedList::new();
  let mut list2_itr = list2.iter();

  for i1 in list1 {
    if list2_itr.len() <= 0 {
      break;
    }

    let i2 = list2_itr.next().unwrap();

    result_list.push_back(*i1);
    result_list.push_back(*i2);
  }
  return result_list;
}
