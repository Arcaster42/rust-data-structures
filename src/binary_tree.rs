#[derive(Debug)]
struct BinaryTree<T> {
  root: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
  value: T,
  left: BinaryTree<T>,
  right: BinaryTree<T>
}

impl<T: PartialOrd> BinaryTree<T> {
  fn new(value: Option<T>) -> Self {
    match value {
      Some(value) => {
        BinaryTree { root: Some(Box::new(Node { value: value, left: BinaryTree { root: None }, right: BinaryTree { root: None } })) }
      }
      None => {
        BinaryTree { root: None }
      }
    }
  }
  
  fn insert(&mut self, value: T) {
    match self.root {
      Some(ref mut node) => {
        if value < node.value {
          node.left.insert(value);
        } else {
          node.right.insert(value);
        }
      }
      None => {
        self.root = Some(Box::new(Node { value: value, left: BinaryTree { root: None }, right: BinaryTree { root: None } }))
      }
    }
  }

  fn contains(&mut self, value: T) -> bool {
    match self.root {
      Some(ref mut node) => {
        if node.value == value {
          return true;
        }
        else if value < node.value {
          return node.left.contains(value);
        }
        else {
          return node.right.contains(value);
        }
      }
      None => {
        return false;
      }
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_value() {
        let tree = BinaryTree::new(Some(5));
        assert_eq!(tree.root.unwrap().value, 5);
    }

    #[test]
    fn test_new_without_value() {
        let tree = BinaryTree::<i32>::new(None);
        assert!(tree.root.is_none());
    }

    #[test]
    fn test_insert() {
        let mut tree = BinaryTree::new(Some(5));
        tree.insert(3);
        tree.insert(7);
        assert_eq!(tree.root.as_ref().unwrap().left.root.as_ref().unwrap().value, 3);
        assert_eq!(tree.root.as_ref().unwrap().right.root.as_ref().unwrap().value, 7);
    }

    #[test]
    fn test_contains() {
        let mut tree = BinaryTree::new(Some(5));
        tree.insert(3);
        tree.insert(7);
        assert!(tree.contains(5));
        assert!(tree.contains(3));
        assert!(tree.contains(7));
        assert!(!tree.contains(8));
    }
}