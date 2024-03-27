#[derive(Debug)]
struct Rectangle {
    x: i32,
    y: i32,
}

fn main() {
    let x = vec![1, 2, 3, 5];

    for j in x {
        println!("{j}");
    }

    let p = Person {
        name: String::from("Danawan"),
        age: 1000,
    };

    let mut manage: Vec<Person> = Vec::new();

    manage.push(p);

    let c = Company {
        name: "Tkpedei".to_string(),
        company_type: CompanyType::Tbk,
        management: manage,
    };

    println!("{}", c.is_tbk());

    let kotak = Rectangle { x: 6, y: 7 };
    println!("{:?}", kotak);
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

#[derive(Debug)]
struct Company {
    name: String,
    company_type: CompanyType,
    management: Vec<Person>,
}

#[derive(Debug)]
enum CompanyType {
    Inc,
    Gmbh(String),
    Ltd,
    Tbk,
}

trait Tbk {
    fn is_tbk(&self) -> bool;
    fn company_name(&self) -> String;
    fn print_management(&self);
    fn print_gmbh(&self);
}

impl Tbk for Company {
    fn is_tbk(&self) -> bool {
        match self.company_type {
            CompanyType::Inc => false,
            CompanyType::Ltd => false,
            CompanyType::Gmbh(_) => false,
            CompanyType::Tbk => true,
        }
    }

    fn company_name(&self) -> String {
        self.name.clone()
    }

    fn print_management(&self) {
        println!("{:?}", self.management);
    }

    fn print_gmbh(&self) {}
}
