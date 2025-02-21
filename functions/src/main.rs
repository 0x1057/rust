fn main() {
    another_function(5, 'z');
    another_function(20, 'a');

    let x = five();

    println!("The value of x is: {x}")
}

fn another_function(x: i32, unit_label: char) {
    println!("{x} : {unit_label}");
}

fn five() -> i32 {
    5
}