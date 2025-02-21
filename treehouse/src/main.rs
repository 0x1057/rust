use std::io::stdin;

#[derive(Debug)]

enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn main() {
    loop {
        println!("Hello what's your name? (Leave empty and press ENTER to quit!");
        let name = what_is_your_name();
        break;

        
    }

    let mut visitor_list = vec![
        Visitor::new("Bert", "AYY", VisitorAction::Accept, 45),
        Visitor::new("Steve", "AYY", VisitorAction::AcceptWithNote{
            note: String::from("Lactose-free milk is in the fridge")
        }, 15),
        Visitor::new("Fred", "AYY", VisitorAction::Refuse, 30),
    ];

    println!("The final list of visitors:");
    println!("{}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}
