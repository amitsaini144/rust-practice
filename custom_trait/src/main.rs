
pub trait Printable {
    fn default_print(&self){
        println!("Default function for printable");
    }

    fn will_be_overridden(&self){
        println!("Default function for will_be_overridden");
    }
}

struct Person {
    name: String,
    age: u16
}

impl Printable for Person{
    fn will_be_overridden(&self) {
        println!("Now this is overwritten, and person's name is {}, and age is {}", self.name, self.age);
    }
}

fn main() {
    let p = Person{
        name: String::from("Amit"),
        age: 23
    };

    p.default_print();
    p.will_be_overridden();
}
