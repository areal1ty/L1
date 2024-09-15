trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

pub fn run() {
    let person = Person {
        name: String::from("Alice"),
    };
    person.say();
}
