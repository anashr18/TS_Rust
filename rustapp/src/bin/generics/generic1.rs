struct Cat {
    language: String,
    age: u8,
}
struct Dog {
    language: String,
    age: u8,
}
struct Biologist {
    desc: String,
    experience: u8,
}
pub trait Details {
    fn info(&self);
}
impl Details for Cat {
    fn info(&self) {
        println!(
            "A Cat that speaks {} and is {} years old",
            self.language, self.age
        );
    }
}
impl Details for Dog {
    fn info(&self) {
        println!(
            "A Dog that speaks {} and is {} years old",
            self.language, self.age
        );
    }
}
pub trait Animal {
    fn make_animal(&self) -> String;
}
impl Animal for Cat {
    fn make_animal(&self) -> String {
        format!(
            "A Cat that speaks {} and is {} years old",
            self.language, self.age
        )
    }
}
impl Animal for Dog {
    fn make_animal(&self) -> String {
        format!(
            "A Dog speaks {} and is {} years old",
            self.language, self.age
        )
    }
}
pub trait Describing<T> {
    fn animal_details(&self, animal: &T);
}

impl<T> Describing<T> for Biologist
where
    T: Animal,
{
    fn animal_details(&self, animal: &T) {
        println!(
            "Biologist with {}  years of experience describes the {:?} as {}",
            self.experience,
            animal.make_animal(),
            self.desc
        )
    }
}
fn main() {
    let cat = Cat {
        language: "meow meow".to_string(),
        age: 5,
    };
    let dog = Dog {
        language: "woof woof".to_string(),
        age: 7,
    };
    cat.info();
    dog.info();
    let bio = Biologist {
        experience: 15,
        desc: "A friendly animal".to_string(),
    };
    bio.animal_details(&cat);
    bio.animal_details(&dog);
}
