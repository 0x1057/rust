fn main() {
    let number = 8;
    let condition = true;
    let numeral = if condition { 5 } else { 6 };

    if number < 5 {
        println!("number is less than 5");
    } else if number > 5 {
        println!("number is more than 5");
    } else {
        println!("number is 5")
    }

    println!("The value of numeral is: {numeral}");

    // loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut new_num = 3;

    while new_num != 0 {
        println!("{new_num}!");
        new_num -= 1;
    }
    println!("LIFTOFF!");

    // for
    let new_arr = [10, 20, 30, 40, 50];

    for element in new_arr {
        println!("The value is: {element}");
    }

    // for with Range
    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF AGAIN!!");
}
