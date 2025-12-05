use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Node<T> {
  data: T,
  next: T,
}

fn random_stat_buff() -> u8 {
    // could actually return some random value here - let's just return
    // some fixed value for now
    42
}
pub fn add_node()-> Node<T>{
    let mut s = HashMap::new();
     s.entry("defence").or_insert_with(random_stat_buff);
     return Node{
    data: s[&0],
    next: s[&1]
     }
}


