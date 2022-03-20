use std::collections::LinkedList;

pub struct Queue<T> {
    elements: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            element: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.element.push_back(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.element.pop_front()
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.element.front()
    }

    pub fn len(&self) -> usize {
        self.element.len()
    }

    pub fn is_empty(&self) -> bool {
        self.element.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue::new()
    }
}

mod tests {
    use super::Queue;

    fn test_enqueue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(87);
        assert_eq!(queue.is_empty(), false);
    }

    fn test_dequeue() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(16);
        queue.enqueue(32);
        let retrieved_dequeue = queue.dequeue();
        assert_eq!(retrieved_dequeue, Some(32));
    }

    fn test_peek_front() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(4);
        queue.enqueue(8);
        let retrieved_peek = queue.peek_front();
        assert_eq!(retrieved_peek, Some(&8));
    }

    fn test_size() {
        let mut queue: Queue<u8> = Queue::new();
        queue.enqueue(2);
        queue.enqueue(4);
        assert_eq!(2, queue.len());
    }
}