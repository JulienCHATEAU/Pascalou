use std::io;
use std::collections::HashMap;

fn all_subset_sum(numbers: Vec<u64>) -> HashMap<u64, u64> {
    let mut res: HashMap<u64, u64> = HashMap::new();
    let array_len = numbers.len();
    let stop = 1 << array_len;
    for subset in 0..stop {
        let mut sum = 0;
        for j in 0..array_len {
            if subset & (1 << j) != 0 {
                sum += numbers[j];
            }
        }
        if res.contains_key(&sum) {
            res.insert(sum, res.get(&sum).unwrap() + 1);
        } else {
            res.insert(sum, 1);
        }
    }
    return res;
}

fn main() {
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).expect("Can't read line !");
    let split: Vec<&str> = first_input.split(" ").collect();
    let n: u64 = split[0].trim().parse().expect("Give a number !");
    let x: u64 = split[1].trim().parse().expect("Give a number !");

    let mut array = String::new();
    io::stdin().read_line(&mut array).expect("Can't read line !");
    let split_array: Vec<u64> = array.split_whitespace().map(|yo| yo.parse().expect("parse error")).collect();

    let (left, right): (&[u64], &[u64]) = split_array.split_at((n/2) as usize);
    let left_sub_sum: HashMap<u64, u64> = all_subset_sum(left.to_vec());
    let right_sub_sum: HashMap<u64, u64> = all_subset_sum(right.to_vec());

    let mut count = 0;
    for left_elem in left_sub_sum.keys() {
        if left_elem <= &x {
            let right_elem = &x-left_elem;
            if right_sub_sum.contains_key(&(right_elem)) {
                count += left_sub_sum.get(&left_elem).unwrap() * right_sub_sum.get(&right_elem).unwrap();
            }
        }
    }
    println!("{}", count)
}