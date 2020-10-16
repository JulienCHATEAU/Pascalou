use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read input");
    let res: Vec<u64> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let n: u64 = res[0];
    let x: u64 = res[1];

    buffer.clear();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read input");
    let values: Vec<u64> = buffer
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let (h1, h2) = values.split_at((n / 2) as usize);

    let sums1 = all_sums(&h1.to_vec());
    let sums2 = all_sums(&h2.to_vec());

    let mut res = 0;

    for k1 in sums1.keys() {
        if k1 <= &x {
            let target = x - k1;
            if sums2.contains_key(&target) {
                res += sums1.get(k1).unwrap() * sums2.get(&target).unwrap();
            }
        }
    }
    print!("{}", res);
}

fn all_sums(set: &Vec<u64>) -> HashMap<u64, u64> {
    let mut res: HashMap<u64, u64> = HashMap::new();
    let array_len = set.len();
    let stop = 1 << array_len;

    for subset in 0..stop {
        let mut sum = 0;
        for j in 0..array_len {
            if subset & (1 << j) != 0 {
                sum += set[j];
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
