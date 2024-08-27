use std::{fmt, mem};

struct Node<T> {
    value: Option<T>,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: Option<T>, next: Option<Box<Node<T>>>) -> Self {
        Self { value, next }
    }
}

pub struct LinkedList<T> {
    head: Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: Node::new(None, None), len: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, value: T) {
        let mut next = None;
        mem::swap(&mut next, &mut self.head.next);

        let node = Node::new(Some(value), next);
        self.head.next = Some(Box::new(node));
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut curr = &mut self.head;
        while let Some(_) = curr.next {
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = Some(Box::new(Node::new(Some(value), None)));
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let mut next = None;
        mem::swap(&mut next, &mut self.head.next.as_mut().unwrap().next);

        let mut node = None;
        mem::swap(&mut node, &mut self.head.next);

        self.head.next = next;
        self.len -= 1;

        Some(node.unwrap().value.unwrap())
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let mut curr = &mut self.head;
        assert!(curr.next.is_some());

        while let Some(_) = curr.next.as_ref().unwrap().next {
            curr = curr.next.as_mut().unwrap();
        }

        let mut node = None;
        mem::swap(&mut node, &mut curr.next);
        self.len -= 1;

        Some(node.unwrap().value.unwrap())
    }
}

impl<T: Clone> From<&[T]> for LinkedList<T> {
    fn from(slice: &[T]) -> Self {
        let mut list = Self::new();
        for i in slice {
            list.push_back(i.clone());
        }
        list
    }
}

impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::from("[");
        let mut curr = &self.head;
        while let Some(_) = curr.next {
            curr = curr.next.as_ref().unwrap();
            output += &format!("{:?}, ", curr.value.as_ref().unwrap());
        }
        output += "]";

        writeln!(f, "{output}")
    }
}
