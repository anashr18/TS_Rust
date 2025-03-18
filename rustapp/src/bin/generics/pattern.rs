fn option_match<T>(option: &Option<T>) {
    match option {
        Some(_v) => println!("some "),
        None => println!("none"),
    }
}

fn main() {
    let option = Some(10);
    option_match(&option);
}
