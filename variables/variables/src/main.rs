fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is {x}");
    // }
    // println!("The value of x is: {x}");

    // Floating point types
    let x = 2.0; 
    let y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.2;
    let product = 4 * 38;
    let quotient = 56.7 / 32.2;
    let truncated_quotient = -5 / 3;
    let remainder = 43 % 5;

    // bool
    let t = true;
    let f: bool = false;

    // char types
    let c = 'z';
    let z: char = 'Z';
    let kitty_cat =  '😻'; // emojis are chars in rust, like swift

    // tuples - below with specific annotations of types of each element
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // tup binds to entire () not each ele. To get each ele must destructure
    let (first, second, third) = tup;
    println!("The value of first: {first}, second: {second}, third: {third}");
    let five_hundred = tup.0;
    println!("{}", five_hundred);

    // arrays - all ele must be same type (tuples ele can be diff types)
    let new_arr: [i32; 5] = [1, 2, 3, 4, 5];
    let fav_months = ["april", "september", "OCTOBER", "november", "august", "december"];

    // access arr ele
    let first_ele = new_arr[0];
    println!("{}", first_ele);
    another_function();
}

fn another_function() {
    println!("Another function.");
}