#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannot divied by zero".to_string());
    }
    Res::Thing(a / b)
}

fn divide_proper(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err(format!("Cannot divide by zero -> {}", b));
    }
    Result::Ok(a / b)
}

#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
}

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Person {
    pub fn print(&self) -> String {
        format!(
            "name = {}, age = {} has {} children",
            self.name, self.age, self.children
        )
    }
}

#[derive(Debug)]
pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr <= self.max {
            let result = Some(self.curr);
            self.curr += self.step;
            result
        } else {
            None
        }
    }
}

// impl IntoIterator for Stepper {
//     type Item = i32;
//     type IntoIter = Self;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self
//     }
// }

fn main() {
    let mut st = Stepper {
        curr: 0,
        step: 2,
        max: 20,
    };

    loop {
        match st.next() {
            Some(expr) => println!("loop value {}", expr),
            None => break,
        }
    }

    let mut rr = Stepper {
        curr: 1,
        step: 2,
        max: 10,
    };

    while let Some(n) = rr.next() {
        println!("while let => {}", n);
    }

    let p = Person {
        name: "matt".to_string(),
        age: 35,
        children: 4,
    };

    println!("Hello, people, from {}", p.print());
    println!("{:?}", p);

    let c = Color::Red;

    match c {
        Color::Red => println!("Red {:?}", c),
        Color::Blue => println!("Blue {:?}", c),
        Color::Green => println!("Green {:?}", c),
    }

    println!("a = {:?} b = {:?}", divide(4, 5), divide(10, 0));

    match divide(10, 5) {
        Res::Thing(v) => println!("val -> {}", v),
        _ => {}
    }

    if let Res::Error(x) = divide(10, 0) {
        println!("the error is {}", x);
    }

    match divide_proper(10, 0) {
        Result::Ok(x) => println!("Answer -> {}", x),
        Result::Err(y) => println!("Error is {}", y),
    }

    let mut m = 0;
    loop {
        m += 1;
        if m == 10 {
            break;
        }
        println!("looper");
    }
}
