use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}
impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        // no use of semicolon and no return
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}
struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}
impl<T: std::fmt::Debug + Clone> LinkedList<T> {
    fn new(value: T) -> Self {
        let node = Node::new(value);
        LinkedList {
            head: Some(node.clone()),
            tail: Some(node),
            length: 1,
        }
    }
    fn append(&mut self, value: T) {
        let new_node = Node::new(value);
        if self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            self.length += 1;
            return;
        }
        let mut current = self.head.clone();
        while let Some(node) = current {
            let mut node_ref = node.borrow_mut();
            if node_ref.next.is_none() {
                node_ref.next = Some(new_node.clone());
                self.tail = Some(new_node);
                self.length += 1;
                return;
            } else {
                current = node_ref.next.clone();
            }
        }
    }
    fn prepend(&mut self, value: T) {
        let new_node = Node::new(value);
        if self.head.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
            self.length += 1;
            return;
        }
        new_node.borrow_mut().next = self.head.clone();
        self.head = Some(new_node);
        self.length += 1;
    }
    fn delete_last(&mut self) {
        // if self.head.is_none() {
        //     return;
        // }
        if let Some(head) = self.head.clone() {
            let mut prev: Link<T> = None;
            let mut current = Some(head.clone());

            while let Some(node) = current {
                let node_ref = node.borrow();
                if node_ref.next.is_none() {
                    if let Some(prev_node) = prev {
                        prev_node.borrow_mut().next = None;
                        self.tail = Some(prev_node);
                        self.length -= 1;
                        return;
                    }
                    // when there is only one node
                    else {
                        self.head = None;
                        self.tail = None;
                        self.length -= 1;
                    }
                }
                prev = Some(node.clone());
                current = node.borrow().next.clone();
            }
        }
    }
    fn delete_first(&mut self) {
        if let Some(head) = self.head.take() {
            self.head = head.borrow().next.clone();
        }
    }
    fn get(&self, idx: usize) {
        let mut current = self.head.clone();
        for _ in 0..idx {
            // println!("{}", i);
            current = current.and_then(|node| node.borrow().next.clone());
        }
        match current {
            Some(node) => {
                // here the errors come as the value is primitive type T which does not implement clone
                // let value = &node.borrow().value.clone();
                let value = &node.borrow().value;
                println!("{:?}", value);
            }
            None => println!("Wrong index"),
        }
    }
    fn set(&mut self, idx: usize, value: T) -> Option<T> {
        let mut current = self.head.clone();
        // ineffcient way to loop
        // for _ in 0..idx {
        //     current = current.and_then(|node| node.borrow().next.clone());
        // }
        for _ in 0..idx {
            if let Some(node) = current {
                current = node.borrow().next.clone();
            } else {
                break;
            }
        }
        // first option
        // if let Some(node) = current {
        //     node.borrow_mut().value = value;
        //     // have to include clone trait in impl of struct for the below line to work
        //     // it performs deep copy of value -primitive type
        //     Some(node.borrow().value.clone())
        // } else {
        //     None
        // }
        // second option
        if let Some(node) = current {
            let mut borrowed_node = node.borrow_mut();
            let old_value = std::mem::replace(&mut borrowed_node.value, value);
            Some(old_value)
        } else {
            None
        }
    }
    fn insert_node(&mut self, idx: usize, value: T) -> Option<T> {
        let new_node = Node::new(value.clone());

        // Special case: inserting at the head
        if idx == 0 {
            new_node.borrow_mut().next = self.head.clone();
            self.head = Some(new_node);
            return Some(value);
        }

        let mut current = self.head.clone();

        // Traverse to the node before the insertion point
        for _ in 0..idx - 1 {
            if let Some(rc_node) = current {
                current = rc_node.borrow().next.clone();
            } else {
                // Index is out of bounds
                return None;
            }
        }

        // Perform the insertion
        if let Some(ref rc_node) = current {
            let next = rc_node.borrow_mut().next.clone();
            new_node.borrow_mut().next = next;
            rc_node.borrow_mut().next = Some(new_node);
            Some(value)
        } else {
            // Index is out of bounds
            None
        }
    }
    fn delete_node(&mut self, idx: usize) {
        let mut current = self.head.clone();

        // for idx =0
        if idx == 0 {}

        for _ in 0..idx - 1 {
            if let Some(rc_node) = current {
                current = rc_node.borrow().next.clone();
            } else {
                // Index is out of bounds
                // return None;
            }
        }

        if let Some(node) = current {
            let next_node = node.borrow().next.clone();
            if let Some(next_node_ref) = next_node {
                node.borrow_mut().next = next_node_ref.borrow().next.clone();
                // Some(idx)
            } else {
                node.borrow_mut().next = None;
            }
        } else {
            // Some(idx)
        }
    }
}
impl<T: std::fmt::Debug> LinkedList<T> {
    fn print_list(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            let node_ref = node.borrow();
            print!("{:?}->", node_ref.value);
            current = node_ref.next.clone();
        }
        println!("None");
    }
}
fn main() {
    println!("Hello from LL in rust");
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
    ll.prepend(22);
    ll.print_list();
    ll.prepend(2299);
    ll.print_list();
    ll.get(7);
    ll.get(0);
    ll.get(3);
    ll.get(4);
    ll.set(3, 999);
    ll.print_list();
    ll.insert_node(4, 100);
    ll.print_list();
    ll.insert_node(5, 200);
    ll.print_list();
    ll.insert_node(6, 300);
    ll.print_list();
    ll.delete_node(2);
    ll.print_list();
    ll.delete_node(0);
    ll.print_list();
    ll.delete_node(0);
    ll.print_list();
    // ll.delete_last();
    // ll.print_list();
    // ll.delete_last();
    // ll.print_list();
    // ll.delete_last();
    // ll.print_list();
    // ll.delete_last();
    // ll.print_list();
    // ll.delete_first();
    // ll.print_list();
}
