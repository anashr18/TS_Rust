// struct Container<T> {
//     value: T,
// }
struct Cat {
    name: String,
    age: u8,
}
struct Dog {
    name: String,
    age: u8,
}
struct PHD {
    experience: u8,
    desc: String,
}
struct Engineer {
    experience: u8,
    desc: String,
}

pub trait Animal {
    fn details(&self) -> String;
}

impl Animal for Dog {
    fn details(&self) -> String {
        format!("This is {} years old Dog name {}", self.age, self.name)
    }
}
impl Animal for Cat {
    fn details(&self) -> String {
        format!("This is {} years old Cat name {}", self.age, self.name)
    }
}
pub trait SelfDescribing<T> {
    fn describe(&self, animal: &T) -> String;
}
impl<T> SelfDescribing<T> for PHD
where
    T: Animal,
{
    fn describe(&self, animal: &T) -> String {
        format!(
            " PHD guy has {} years of experience and describes the animal {} as {}",
            self.experience,
            animal.details(),
            self.desc
        )
    }
}
impl<T> SelfDescribing<T> for Engineer
where
    T: Animal,
{
    fn describe(&self, animal: &T) -> String {
        format!(
            " Engineer guy has {} years of experience and describes the animal {} as {}",
            self.experience,
            animal.details(),
            self.desc
        )
    }
}
fn describe_type<T, U>(t: &T, describer: &U) -> String
where
    U: SelfDescribing<T>,
{
    describer.describe(t)
}
fn main() {
    let dog = Dog {
        name: "dog1".to_string(),
        age: 20,
    };
    let cat = Cat {
        name: "cat1".to_string(),
        age: 10,
    };
    let dr = PHD {
        experience: 12,
        desc: "nice agile dog".to_string(),
    };
    let eng = Engineer {
        experience: 8,
        desc: "good looking dog".to_string(),
    };

    println!("{}", describe_type(&dog, &dr));
    println!("{}", describe_type(&dog, &eng));
    println!("{}", describe_type(&cat, &dr));
    println!("{}", describe_type(&cat, &eng));
    // let str_container = Container {
    //     value: Some("hello hi"),
    // };
    // // let str_container = Container {
    // //     value: Option::<&str>::None,
    // // };
    // match str_container.value {
    //     Some(val) => println!("{}", val),
    //     None => println!("{}", "None"),
    // }
}
