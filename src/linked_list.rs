struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self { value, next: None }
    }
}

pub struct LinkedList {
    head: Node, // dummy
    len: usize,
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: Node::new(0), len: 0 }
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

    pub fn push_back(&mut self, value: i32) {
        let mut curr = &mut self.head;
        while let Some(_) = curr.next {
            curr = curr.next.as_mut().unwrap();
        }
        curr.next = Some(Box::new(Node::new(value)));
        self.len += 1;
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