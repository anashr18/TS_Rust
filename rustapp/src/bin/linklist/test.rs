use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let data = Rc::new(RefCell::new(5));

    let rc_clone = Rc::clone(&data);
    {
        // Clone Rc for shared ownership
        {
            // Immutable borrow from Rc<RefCell>
            let borrow = rc_clone.borrow();
            println!("Immutable borrow: {}", borrow);
            // borrow ends here
        }
        // Mutable borrow is now safe
        let mut mutable_borrow = data.borrow_mut();
        *mutable_borrow += 1;
        println!("Mutable borrow after increment: {}", mutable_borrow);
    }

    // Original Rc still valid
    println!("Final value: {}", data.borrow());
    println!("Final value: {}", rc_clone.borrow());
}
