use std::io;

fn main() {
    io::stdin().read_line(&mut String::new()).expect("Can't read line !");

    let mut list = String::new();
    io::stdin().read_line(&mut list).expect("Can't read line !");
    let split = list.split(" ");

    let mut res = 0;
    let mut nmoinsun = 0;
    for i in split {
        let mut number: i64 = i.trim().parse().expect("Give a number !");
        if number < nmoinsun {
            let sub = nmoinsun - number;
            number += sub;
            res += sub;
        }
        nmoinsun = number;
    }

    println!("{}", res);
}
