pub struct StackStruct {
  name: String,
  stack: Vec<i32>
}

impl StackStruct {
  fn new(name: String) -> Self {
    Self { name: name, stack: Vec::new() }
  }

  fn get_name(&self) -> String {
    return self.name.clone();
  }

  fn push(&mut self, value: i32) {
    self.stack.push(value);
  }

  fn pop(&mut self) -> Option<i32> {
    return self.stack.pop();
  }
}

fn main() {
  
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_instantiation() {
    let test_struct = StackStruct::new("test".to_string());
    assert_eq!(test_struct.get_name(), "test".to_string());
  }

  #[test]
  fn test_pushes() {
    let mut test_struct = StackStruct::new("test".to_string());
    test_struct.push(1);
    test_struct.push(2);
    assert_eq!(test_struct.stack.len(), 2);
    assert_eq!(test_struct.pop(), Some(2));
  }

  #[test]
  fn test_pushes_and_pops() {
    let mut test_struct = StackStruct::new("test".to_string());
    test_struct.push(1);
    test_struct.push(2);
    test_struct.push(3);
    test_struct.pop();
    test_struct.push(4);
    test_struct.push(5);
    test_struct.pop();
    assert_eq!(test_struct.stack.len(), 3);
    assert_eq!(test_struct.stack[1], 2);
    assert_eq!(test_struct.stack[2], 4);
  }

  #[test]
  fn test_pop_empty_stack() {
      let mut test_struct = StackStruct::new("test".to_string());
      assert_eq!(test_struct.pop(), None);
  }

  #[test]
  fn test_pop_nonempty_stack() {
      let mut test_struct = StackStruct::new("test".to_string());
      test_struct.push(1);
      test_struct.push(2);
      assert_eq!(test_struct.pop(), Some(2));
      assert_eq!(test_struct.pop(), Some(1));
      assert_eq!(test_struct.pop(), None);
  }

  #[test]
  fn test_push_overflow() {
      let mut test_struct = StackStruct::new("test".to_string());
      for i in 0..1000 {
          test_struct.push(i);
      }
      assert_eq!(test_struct.stack.len(), 1000);
  }

  #[test]
  fn test_pop_underflow() {
      let mut test_struct = StackStruct::new("test".to_string());
      for i in 0..1000 {
          test_struct.push(i);
      }
      for i in 0..1000 {
          assert_eq!(test_struct.pop(), Some(999 - i));
      }
      assert_eq!(test_struct.pop(), None);
  }
}