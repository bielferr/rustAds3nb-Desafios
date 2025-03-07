

use std::ptr;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    length: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: ptr::null_mut(),
            length: 0,
        }
    }

    pub fn enqueue(&mut self, elem: T) {
        let mut new_node = Box::new(Node {
            value: elem,
            next: None,
        });
        let raw_node: *mut _ = &mut *new_node;

        unsafe {
            if self.tail.is_null() {
                self.head = Some(new_node);
                self.tail = raw_node;
            } else {
                (*self.tail).next = Some(new_node);
                self.tail = raw_node;
            }
        }
        self.length += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            self.length -= 1;
            old_head.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn test_queue_operations() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.peek(), Some(&1));
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.dequeue(), Some(3));
        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
    }
}
