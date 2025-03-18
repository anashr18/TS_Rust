use std::cell::RefCell;
use std::rc::Rc;

type ItemData<T> = Rc<RefCell<T>>;
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;
#[derive(Clone)]
struct ListItem<T> {
    data: ItemData<T>,
    next: Option<ListItemPtr<T>>,
}

impl<T> ListItem<T> {
    fn new(t: T) -> Self {
        ListItem {
            data: Rc::new(RefCell::new(t)),
            next: None,
        }
    }
}
#[derive(Clone)]
struct LinkedList<T> {
    head: ListItemPtr<T>,
    cur_iter: Option<ListItemPtr<T>>,
}
impl<T> LinkedList<T> {
    fn new(t: T) -> Self {
        LinkedList {
            head: Rc::new(RefCell::new(ListItem::new(t))),
            cur_iter: None,
        }
    }
    fn append(&mut self, t: T) {
        self.last()
            .expect("List is empty")
            .as_ref()
            .borrow_mut()
            .next = Some(Rc::new(RefCell::new(ListItem::new(t))));
    }
    fn iter(&self) -> LinkedList<T> {
        LinkedList {
            head: self.head.clone(),
            cur_iter: Some(self.head.clone()),
        }
    }
}
impl<T> Iterator for LinkedList<T> {
    type Item = ListItemPtr<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.cur_iter.clone() {
            None => {
                self.cur_iter = Some(self.head.clone());
            }
            Some(ptr) => {
                self.cur_iter = ptr.borrow().next.clone();
            }
        }
        self.cur_iter.clone()
    }
}
fn main() {
    let mut dinosaurs = LinkedList::new("Tyr rex");
    // let last_item = dinosaurs.clone().last().expect("No last item");
    // println!("last item: {}", last_item.clone().borrow().data.borrow());
    dinosaurs.append("2nd");
    dinosaurs.append("3rd");
    dinosaurs.append("4th");
    dinosaurs.append("5th");
    dinosaurs
        .iter()
        .for_each(|ptr| println!("{}", ptr.borrow().data.borrow()));
    dinosaurs
        .iter()
        .for_each(|ptr| println!("{}", ptr.borrow().data.borrow()));
}
