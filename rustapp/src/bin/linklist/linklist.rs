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
        let node: Rc<RefCell<Node<T>>> = Node::new(value);
        LinkedList {
            head: Some(node.clone()),
            tail: Some(node),
            length: 1,
        }
    }
    fn append(&mut self, value: T) {
        let new_node: Rc<RefCell<Node<T>>> = Node::new(value);
        let mut current = self.head.clone();
        while let Some(node) = current {
            let mut current_node_borrow = node.borrow_mut();
            if current_node_borrow.next.is_none() {
                current_node_borrow.next = Some(new_node.clone());
                self.tail = Some(new_node);
                self.length += 1;
                return;
            }
            current = current_node_borrow.next.clone();
        }
    }
    fn delete_last(&mut self) {
        if self.head.is_none() {
            return;
        }
        let mut current = self.head.clone();
        let mut prev: Link<T> = None;
        while let Some(node) = current {
            let current_node_borrow = node.borrow();
            if current_node_borrow.next.is_none() {
                if let Some(prev_node) = prev {
                    prev_node.borrow_mut().next = None;
                    self.tail = Some(prev_node);
                    self.length -= 1;
                } else {
                    self.head = None;
                    self.tail = None;
                    self.length -= 1;
                }
                return;
            }
            prev = Some(node.clone());
            // current = current_node_borrow.next.clone();
            if let Some(ref next_current) = current_node_borrow.next {
                current = Some(next_current.clone());
            } else {
                current = None;
            }
        }
    }
    // fn prepend(&mut self, value: T) {
    //     let new_node = Node::new(value);

    //     if self.head.is_none() {
    //         self.head = Some(new_node.clone());
    //         self.tail = Some(new_node);
    //     } else {
    //         new_node.borrow_mut().next = self.head.clone(); // Clone here is fine for ownership
    //         self.head = Some(new_node);
    //     }

    //     self.length += 1;
    // }
    fn prepend(&mut self, value: T) {
        // why not ref
        let new_node = Node::new(value);
        if self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            self.length += 1;
            return;
        }
        // why not clone vs borrow(ref)
        new_node.borrow_mut().next = self.head.clone();
        self.head = Some(new_node);
    }
}
impl<T: std::fmt::Debug + Clone> LinkedList<T> {
    // Add Clone here
    fn print_list(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            let value = node.borrow().value.clone(); // Clone the value
            print!("{:?} -> ", value);
            current = node.borrow().next.clone(); // Clone the next pointer
        }
        println!("None");
    }
}

fn main() {
    let mut ll = LinkedList::new(4);
    ll.print_list();
    ll.append(66);
    ll.delete_last();
    ll.print_list();
    ll.append(662);
    ll.print_list();
    ll.append(1166);
    ll.print_list();
    ll.delete_last();
    ll.print_list();
    ll.delete_last();
    ll.delete_last();
    ll.delete_last();
    ll.delete_last();
    ll.delete_last();
    ll.print_list();
}
