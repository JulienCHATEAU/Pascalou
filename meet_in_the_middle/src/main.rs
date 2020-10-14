use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Could not read input");
    let res :Vec<u32> = buffer.trim().split_whitespace().map(|s| s.parse().expect("parse error")).collect();
    let n :u32 = res[0];
    let x :u32 = res[1];

    buffer.clear();
    std::io::stdin().read_line(&mut buffer).expect("Could not read input");
    let values :Vec<u32> = buffer.trim().split_whitespace().map(|s| s.parse().expect("parse error")).collect();

    let (h1, h2) = values.split_at((n/2) as usize);
    let h1 = h1.to_vec();
    let h2 = h2.to_vec();

    let sums1 = all_sums(&h1);
    let sums2 = all_sums(&h2);

    let mut res = 0;

    for e1 in sums1 {
        if sums2.contains_key(&(x-e1.0)) {
            res += e1.1 * sums2.get(&(x-e1.0)).unwrap();
        }
    }
    print!("{}", res);
}

fn all_sums(v :&Vec<u32>) -> HashMap<u32, u32> {
    let mut sums :HashMap<u32, u32> = HashMap::new();
    let subs :Vec<Vec<u32>> = (0..2usize.pow(v.len() as u32)).map(|i| {
        v.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
                            .map(|(_, element)| element.clone())
                            .collect()
    }).collect();
    let mut tmp :u32;
    for sub in subs {
        tmp = sub.iter().sum();
        if sums.contains_key(&tmp) {
            sums.insert(tmp, sums.get(&tmp).unwrap()+1);
        } else {
            sums.insert(tmp, 1);
        }
    }
    return sums;
}
