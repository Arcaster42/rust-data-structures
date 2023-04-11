#[derive(Debug)]
pub struct Queue<T> {
  items: Vec<T>
}

impl<T> Queue<T> {
  fn new() -> Self {
    Self { items: Vec::new() }
  }

  fn enqueue(&mut self, val: T) {
    self.items.push(val);
  }

  fn dequeue(&mut self) -> Option<T> {
    let removed = if self.items.len() > 0 { Some(self.items.remove(0)) } else { None };
    removed
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_queue() {
        let queue: Queue<i32> = Queue::new();
        
        assert_eq!(queue.items, vec![])
    }

    #[test]
    fn test_enqueue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        
        assert_eq!(queue.items, vec![1, 2, 3])
    }

    #[test]
    fn test_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }

    #[test]
    fn test_dequeue_empty_queue() {
        let mut queue: Queue<i32> = Queue::new();
        assert_eq!(queue.dequeue(), None);
    }
}