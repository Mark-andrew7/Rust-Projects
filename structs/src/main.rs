#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

impl Person {
    fn greet(&self) {
        println!("Hello my name is {} and I am {} years old from {}", self.name, self.age, self.city)
    }
}

fn main() {
    let p1 = Person {
        name: String::from("Mark"),
        age: 22,
        city: String::from("Nairobi"),
    };
    p1.greet();
    println!("{:?}", p1);
}