fn main() {
    let x = vec![1, 2, 3, 5];

    for j in x {
        println!("{j}");
    }

    let p = Person {
        name: String::from("Danawan"),
        age: 1000,
    };

    println!("{:?}", p);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn get_name(self) -> String {
        self.name
    }

    fn print_age(&self) {
        println!("the age is {}", self.age);
    }
}
