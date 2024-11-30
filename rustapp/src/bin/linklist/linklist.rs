use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}
struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}
impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}
impl<T> LinkedList<T> {
    fn new(value: T) -> Self {
        let node = Node::new(value);
        LinkedList {
            head: Some(node.clone()),
            tail: Some(node),
            length: 1,
        }
    }
}

fn main() {
    let ll = LinkedList::new(4);
}
