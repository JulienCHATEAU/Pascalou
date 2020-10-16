use std::io;

// fn fact(n: i64) -> i64 {
//     if n == 1 {
//         return 1;
//     }
//     return n * fact(n-1);
// }

fn n_count(n: i64, number: i64) -> i64 {
    if number < n {
        return 0;
    }
    return 1 + n_count(n, number-n);
}

fn trailing_zeros(number: i64) -> i64 {
    let mut n = 5;
    let mut sum = 0;
    while n <= number {
        sum += n_count(n, number);
        n *= 5;
    }
    return sum;
}

fn main() {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Can't read line !");
    let number: i64 = number.trim().parse().expect("Give a number !");

    // for elem in 1..number {
    //     println!("{}! = {}", elem, fact(elem))
    // }

    println!("{}", trailing_zeros(number));
}