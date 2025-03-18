use std::any::Any;

struct MyStruct1;
impl MyStruct1 {
    fn struct_hello(&self) {
        println!("This is a struct1 hello.");
    }
}
struct MyStruct2;
impl MyStruct2 {
    fn struct_hello(&self) {
        println!("This is a struct2 method.");
    }
}
pub trait Mytrait {
    fn trait_hello(&self);
    fn as_any(&self) -> &dyn Any;
}
impl Mytrait for MyStruct1 {
    fn trait_hello(&self) {
        println!("This is hello from trait1 hello");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Mytrait for MyStruct2 {
    fn trait_hello(&self) {
        println!("This is hello from trait2 hello");
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let mut list = Vec::<Box<dyn Mytrait>>::new();
    list.push(Box::new(MyStruct1 {}));
    list.push(Box::new(MyStruct2 {}));
    list.iter().for_each(|f| f.trait_hello());
    list.iter().for_each(|f| {
        if let Some(obj) = f.as_any().downcast_ref::<MyStruct1>() {
            obj.struct_hello();
        }
        if let Some(obj) = f.as_any().downcast_ref::<MyStruct2>() {
            obj.struct_hello();
        }
    });
}
