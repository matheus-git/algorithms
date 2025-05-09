use std::rc::Rc;
use std::cell::RefCell;

type NodeRef = Rc<RefCell<Node>>;

pub struct Node {
    pub value: usize,
    pub next: Option<NodeRef>,
    pub previous: Option<NodeRef>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev_val = self.previous
            .as_ref()
            .map(|p| p.borrow().value)
            .unwrap_or_default();
        let next_val = self.next
            .as_ref()
            .map(|n| n.borrow().value)
            .unwrap_or_default();
        write!(
            f,
            "Node {{ value: {}, previous: {}, next: {} }}",
            self.value, prev_val, next_val
        )
    }
}


pub struct LinkedList {
    pub head: Option<NodeRef>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn insert_front(&mut self, value: usize) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.clone(),
            previous: None,
        }));

        if let Some(old_head) = &self.head {
            old_head.borrow_mut().previous = Some(new_node.clone());
        }

        self.head = Some(new_node);
    }

    pub fn insert(&self, x: NodeRef, y: &NodeRef) {
        let mut y_borrow = y.borrow_mut();
        let mut x_borrow = x.borrow_mut();
        if let Some(next) = &y_borrow.next {
            x_borrow.next = Some(next.clone());
            next.borrow_mut().previous = Some(x.clone());
        }else{
            x_borrow.next = None;
        }
        x_borrow.previous=Some(y.clone());
        y_borrow.next=Some(x.clone());
    }

    pub fn push_head(&mut self, value: usize) -> NodeRef {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            previous: None,
        }));
        self.head = Some(new_node.clone());
        new_node
    }

    pub fn push_after(&self, value: usize, after: &NodeRef) -> NodeRef {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            previous: None,
        }));
        self.insert(new_node.clone(), after);
        new_node
    }
    
    pub fn find(&self, k: usize) -> Option<Rc<RefCell<Node>>> {
        let mut current = self.head.clone();
        while let Some(node_rc) = current {
            if node_rc.borrow().value == k {
                return Some(node_rc.clone());
            }
            current = node_rc.borrow().next.clone();
        }
        None
    }

    pub fn remove(&mut self, x: &NodeRef) {
        let mut x_borrow = x.borrow_mut();

        if let Some(prev) = &x_borrow.previous {
            prev.borrow_mut().next = x_borrow.next.clone();
        } else {
            self.head = x_borrow.next.clone();
        }

        if let Some(next) = &x_borrow.next {
            next.borrow_mut().previous = x_borrow.previous.clone();
        }

        x_borrow.next = None;
        x_borrow.previous = None;
    }
}

impl std::fmt::Debug for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head.clone();
        let mut values = vec![];
        while let Some(node) = current {
            values.push(node.borrow().value);
            current = node.borrow().next.clone();
        }
        write!(f, "{:?}", values)
    }
}
