fn main() {
    for i in 1..=40 {
        println!("{}", n_fibo(i));
    }

    println!("{}", f_to_c(100.0));
}

fn f_to_c(fahrenheit: f64) -> f64 {
    //C = 5/9(F-32)
    5.0 / 9.0 * (fahrenheit - 32.0)
}

fn n_fibo(n: i64) -> i64 {
    //fibonacci is:
    //number + number before equals next number
    let mut number = 1;
    let mut next_number = 1;
    let mut scratch = 0;
    // let number_three = number + next_number;
    // let number_four = next_number + number_three;

    if n == 1 || n == 2 {
        1
    } else {
        for _i in 2..n {
            scratch = next_number;
            next_number += number;
            number = scratch;
        }
        next_number
    }
}
