struct Cookie {
    color: String,
}
impl Cookie {
    fn new(color: &str) -> Self {
        Cookie {
            color: color.to_string(),
        }
    }
    fn get_color(&self) -> &str {
        &self.color
    }
    fn set_color(&mut self, new_color: &str) {
        self.color = new_color.to_string()
    }
}

fn main() {
    println!("hello from rusting");
    let c1 = Cookie::new("black");
    let mut c2 = Cookie::new("white");
    println!("{}", c1.get_color());
    println!("{}", c2.get_color());
    c2.set_color("blue");
    println!("{}", c2.get_color());
}
