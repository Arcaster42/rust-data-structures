#[derive(Debug)]
pub struct LinkedList<T> {
  head: Option<Box<Node<T>>>
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Node<T> {
  value: Option<T>,
  next: Option<Box<Node<T>>>
}

impl<T: PartialEq> LinkedList<T> {
  pub fn new() -> Self {
    Self { head: Some(Box::new(Node { value: None, next: None })) }
  }

  pub fn add_node(&mut self, value: T) {
    let mut current_node = &mut self.head;
    loop {
      match current_node {
        Some(node) => {
          if node.next.is_some() {
            current_node = &mut node.next;
          } else {
            node.next = Some(Box::new(Node { value: Some(value), next: None }));
            break;
          }
        }
        None => {
          break;
        }
      }
    }
  }

  fn remove_node(&mut self, value: &T) {
    let mut current_node = &mut self.head;
    loop {
      match current_node {
        Some(node) => {
          if node.next.is_some() {
            if node.next.as_ref().unwrap().value == *value {
              node.next = node.next.as_mut().unwrap().next.take();
              break;
            }
          }
        }
        None => {
          break;
        }
      }
      current_node = &mut current_node.as_mut().unwrap().next;
    }
  }

  fn get_node(&self, index: i16) -> Option<&Box<Node<T>>> {
    let mut current_node = &self.head;
    let mut current_index = 0;
    loop {
      match current_node {
        Some(node) => {
          if current_index == index {
            return Some(node)
          } else {
            current_index += 1;
            current_node = &node.next;
          }
        }
        None => {
          return None;
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_linked_list() {
      let mut list: LinkedList<i32> = LinkedList::new();
      list.add_node(1);
      assert_eq!(list.head.unwrap().value.unwrap(), 1);
  }

  #[test]
  fn test_add_node_to_linked_list() {
      let mut list: LinkedList<i32> = LinkedList::new();
      list.add_node(1);
      list.add_node(2);
      list.add_node(3);
      let mut current_node = list.head.unwrap();
      assert_eq!(current_node.value.unwrap(), 1);
      current_node = current_node.next.unwrap();
      assert_eq!(current_node.value.unwrap(), 2);
      current_node = current_node.next.unwrap();
      assert_eq!(current_node.value.unwrap(), 3);
  }

  #[test]
  fn test_empty_linked_list() {
      let list: LinkedList<i32> = LinkedList { head: None };
      assert_eq!(list.head, None);
  }

  #[test]
  fn test_remove_node_from_linked_list() {
      let mut list: LinkedList<i32> = LinkedList::new();
      list.add_node(1);
      list.add_node(2);
      list.add_node(3);
      list.remove_node(2);
      let mut current_node = list.head.unwrap();
      assert_eq!(current_node.value.unwrap(), 1);
      current_node = current_node.next.unwrap();
      assert_eq!(current_node.value.unwrap(), 3);
  }

  #[test]
  fn test_get_node_by_index() {
      let mut list: LinkedList<i32> = LinkedList::new();
      list.add_node(1);
      list.add_node(2);
      list.add_node(3);
      assert_eq!(list.get_node(1).unwrap().value.unwrap(), 2);
  }

  #[test]
  fn test_get_node_by_index_out_of_bounds() {
      let list: LinkedList<i32> = LinkedList::new();
      assert_eq!(list.get_node(1), None);
  }
}