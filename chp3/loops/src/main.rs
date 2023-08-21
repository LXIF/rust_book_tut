fn main() {
    //LOOP
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

    //WHILE
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //while can also be used for iteration! though not recced.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //for

    let a = ["wargle", "bargle", "schnagrle", "gargle"];

    for element in a {
        println!("borgle? nargle: {}", element);
    }

    //for with range

    for n in (1..=10).rev() {
        println!("{n}!");
    }
    println!("LORFTORF?");
}