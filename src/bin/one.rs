#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    pub fn print(&self) {
        println!("Hi there ! {}", self.name);
    }

    pub fn age_up(&mut self, x: i32) {
        self.age += x;
    }

    pub fn drop_me(self) {}
}

pub fn get_age(s: &Person) -> &i32 {
    &s.age
}

fn main() {
    let mut p = Person::new("Matt".to_string(), 123);

    let a = get_age(&p);

    println!("{}", a);
    p.print();
    p.print();

    p.age_up(12);
    p.drop_me();
}
