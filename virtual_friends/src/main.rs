use std::collections::HashMap;

struct UnionFind {
    ids: Vec<u32>,
    sizes: Vec<u32>,
    count: u32,
}

impl UnionFind {
    fn new(n: u32) -> UnionFind {
        UnionFind {
            count: n,
            sizes: vec![1; n as usize],
            ids: (0..n).collect(),
        }
    }

    // fn nb_components(&self) -> u32 {
    //     self.count
    // }

    fn size_set(&mut self, i: u32) -> u32 {
        let index = self.find_set(i);
        return self.sizes[index as usize];
    }

    fn find_set(&mut self, i: u32) -> u32 {
        if self.ids[i as usize] == i {
            return i;
        } else {
            self.ids[i as usize] = self.find_set(self.ids[i as usize]);
            return self.ids[i as usize];
        }
    }

    // fn connected(&mut self, i: u32, j: u32) -> bool {
    //     self.find_set(i) == self.find_set(j)
    // }

    fn union_set(&mut self, p: u32, q: u32) {
        let i: u32 = self.find_set(p);
        let j: u32 = self.find_set(q);
        if i == j {
            return;
        }
        if self.sizes[i as usize] < self.sizes[j as usize] {
            self.ids[i as usize] = j;
            self.sizes[j as usize] += self.sizes[i as usize];
        } else {
            self.ids[j as usize] = i;
            self.sizes[i as usize] += self.sizes[j as usize];
        }
        self.count -= 1;
    }
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Could not read input");
    let nb_tests: u32 = buffer.trim().parse().expect("Could not convert to int");

    let mut tests: Vec<Vec<Vec<String>>> = Vec::new();
    let mut maps: Vec<HashMap<String, u32>> = Vec::new();

    for test_index in 0usize..nb_tests as usize {
        tests.push(Vec::new());
        maps.push(HashMap::new());
        buffer.clear();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Could not read input");
        let nb_relations: u32 = buffer.trim().parse().expect("Could not convert");
        for relation_index in 0usize..nb_relations as usize {
            tests[test_index].push(Vec::new());
            buffer.clear();
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Could not read input");
            tests[test_index][relation_index] = buffer
                .split_whitespace()
                .map(|word| word.to_string())
                .collect();

            let mut person: String = tests[test_index][relation_index][0].clone();
            if !maps[test_index].contains_key(&person) {
                let index: u32 = maps[test_index].len() as u32;
                maps[test_index].insert(person, index);
            }
            person = tests[test_index][relation_index][1].clone();
            if !maps[test_index].contains_key(&person) {
                let index: u32 = maps[test_index].len() as u32;
                maps[test_index].insert(person, index);
            }
        }
    }

    for test_index in 0..tests.len() {
        let mut uf = UnionFind::new(maps[test_index].len() as u32);
        let map = &maps[test_index];
        for relation in &tests[test_index] {
            uf.union_set(
                *map.get(&relation[0]).unwrap(),
                *map.get(&relation[1]).unwrap(),
            );
            let i = uf.find_set(*map.get(&relation[0]).unwrap());
            println!("{}", uf.size_set(i));
        }
    }
}
