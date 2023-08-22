use std::mem;
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32, next: Option<Box<Node>>) -> Self {
        Self { value, next }
    }
}

pub struct LinkedList {
    head: Node, // dummy
    len: usize,
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: Node::new(0, None), len: 0 }
    }

    pub fn from(slice: &[i32]) -> Self {
        let mut list = Self::new();
        for i in slice {
            list.push_back(*i);
        }
        list
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, value: i32) {
        let mut next = None;
        mem::swap(&mut next, &mut self.head.next);

        let node = Node::new(value, next);
        self.head.next = Some(Box::new(node));
        self.len += 1;
    }

    pub fn push_back(&mut self, value: i32) {
        let mut curr = &mut self.head;
        while let Some(_) = curr.next {
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = Some(Box::new(Node::new(value, None)));
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let mut next = None;
        mem::swap(&mut next, &mut self.head.next.as_mut().unwrap().next);

        let mut node = None;
        mem::swap(&mut node, &mut self.head.next);

        self.head.next = next;
        self.len -= 1;

        Some(node.unwrap().value)
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        let mut curr = &mut self.head;
        assert!(curr.next.is_some());

        while let Some(_) = curr.next.as_ref().unwrap().next {
            curr = curr.next.as_mut().unwrap();
        }

        let result = Some(curr.next.as_ref().unwrap().value);
        curr.next = None;
        self.len -= 1;

        result
    }

    pub fn print(&self) {
        let mut curr = &self.head;

        print!("[");
        while let Some(_) = curr.next {
            curr = curr.next.as_ref().unwrap();
            print!("{}, ", curr.value);
        }
        println!("]");
    }
}