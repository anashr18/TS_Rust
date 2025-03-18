### while let
```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4];

    while let Some(value) = numbers.pop() {
        println!("Popped: {}", value);
    }

    println!("All elements are removed!");
}
```
### loop match 
```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4];

    while let Some(value) = numbers.pop() {
        println!("Popped: {}", value);
    }

    println!("All elements are removed!");
}
```

### Why no semicolon?
Without a semicolon: The final expression in a function body is returned implicitly. In Rust, the last expression of a function without a semicolon is treated as the return value of the function.

Here, Rc::new(...) is the final expression, so it is returned automatically.

With a semicolon: Adding a semicolon turns the expression into a statement. A statement does not produce a value, so the function body would no longer have a return value, leading to a mismatched types error.
```rust
fn new(value: T) -> Rc<RefCell<Node<T>>> {
    Rc::new(RefCell::new(Node {
        value: value,
        next: None,
    }))
}
fn new(value: T) -> Rc<RefCell<Node<T>>> {
    return Rc::new(RefCell::new(Node {
        value: value,
        next: None,
    }));
}
```

### shared ownership with Rc-refrence counter
Many mut/immut can be created with Rc type.They are different than borrowed refrences mut/immut. 
- in prepand method 
  - these are Rc cloning not refrence cloning 
```rust
let mut prev: Link<T> = self.head.clone();
let mut current = self.head.clone();
```
- borrowed ref 
  - they can exist in scope --immut borrowing
  ```rust
  let node_ref1 = node.borrow();
  let node_ref2 = node.borrow();
  ```
  - They can't
  ```rust
    let node_ref1 = node.borrow_mut();
    let node_ref2 = node.borrow();
  ```
### When to Use `Rc::clone` or `&`

| Scenario                             | Use `Rc::clone` | Use `&` |
| ------------------------------------ | --------------- | ------- |
| Multiple owners required             | ✅               | ❌       |
| Data only needs to be accessed once  | ❌               | ✅       |
| Mutable access required elsewhere    | ✅               | ❌       |
| Avoiding runtime `RefCell` conflicts | ✅               | ❌       |

```rust
// option, Rc, Refcell implements clone trait..hence clone works
let value = &node.borrow().next.clone();
// below errors as the value is primitive type T which does not implement clone trait
let value = &node.borrow().value.clone();
// here we take refrence _this would not be a good approach if we hve to get more ref or mut ref
let value = &node.borrow().value;
// we can also put a clone trait in impl block
impl<T: std::fmt::Debug+clone> LinkedList<T>
// clone works now
let value = &node.borrow().value.clone();
println!("{:?}", value);
```
[Implementing `Copy` and `Clone` Traits in Rust](docs/rust_copy_clone.md)