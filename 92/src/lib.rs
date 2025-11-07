// Define generic node structure
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// ===== TRAITS =====
pub trait Push<T> {
    fn push(self, val: T) -> Self;
}

pub trait Pop<T> {
    fn pop(self) -> (Option<T>, Self);
}

// ===== STACK =====
pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

pub fn stack<T>() -> Stack<T> {
    Stack { top: None }
}

impl<T> Push<T> for Stack<T> {
    fn push(self, val: T) -> Self {
        Stack {
            top: Some(Box::new(Node {
                value: val,
                next: self.top,
            })),
        }
    }
}

impl<T> Pop<T> for Stack<T> {
    fn pop(self) -> (Option<T>, Self) {
        match self.top {
            Some(node) => {
                let Node { value, next } = *node;
                (Some(value), Stack { top: next })
            }
            None => (None, Stack { top: None }),
        }
    }
}

// ===== QUEUE =====
pub struct Queue<T> {
    front: Option<Box<Node<T>>>,
    back: Option<*mut Node<T>>, // raw pointer for O(1) enqueue
}

pub fn queue<T>() -> Queue<T> {
    Queue {
        front: None,
        back: None,
    }
}

impl<T> Push<T> for Queue<T> {
    fn push(mut self, val: T) -> Self {
        let mut new_node = Box::new(Node { value: val, next: None });
        let raw_ptr: *mut _ = &mut *new_node;

        match self.front {
            None => {
                self.front = Some(new_node);
                self.back = Some(raw_ptr);
            }
            Some(mut head) => unsafe {
                // Walk to back and attach
                let mut tail = self.back.unwrap();
                (*tail).next = Some(new_node);
                self.back = Some(raw_ptr);
                self.front = Some(head);
            },
        }
        self
    }
}

impl<T> Pop<T> for Queue<T> {
    fn pop(self) -> (Option<T>, Self) {
        match self.front {
            Some(node) => {
                let Node { value, next } = *node;
                let back = if next.is_none() { None } else { self.back };
                (Some(value), Queue { front: next, back })
            }
            None => (None, Queue { front: None, back: None }),
        }
    }
}








