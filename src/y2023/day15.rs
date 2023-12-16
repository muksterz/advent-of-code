use runner::aoc;

fn hash(input: &str) -> u64 {
    let mut hash = 0;
    for c in input.bytes() {
        if c == b'\n' {
            continue;
        }
        hash += c as u64;
        hash *= 17;
        hash %= 256;
    }

    hash
}

#[aoc(day15, part1)]
fn part1(input: &str) -> u64 {
    let mut total = 0;
    for s in input.split(',') {
        total += hash(s)
    }
    total
}

#[derive(Default, Debug, Clone)]
struct VecMap<K, V> {
    vals: Vec<(K, V)>,
}

impl<K: Eq, V> VecMap<K, V> {
    fn insert(&mut self, k: K, v: V) {
        if let Some(i) = self
            .vals
            .iter()
            .enumerate()
            .find(|(_, (ki, _))| ki == &k)
            .map(|(i, _)| i)
        {
            self.vals[i] = (k, v);
        } else {
            self.vals.push((k, v));
        }
    }

    fn remove(&mut self, k: &K) {
        if let Some(i) = self
            .vals
            .iter()
            .enumerate()
            .find(|(_, (ki, _))| ki == k)
            .map(|(i, _)| i)
        {
            self.vals.remove(i);
        }
    }
    fn values(&self) -> impl Iterator<Item = &V> {
        self.vals.iter().map(|i| &i.1)
    }
}

#[aoc(day15, part2)]
fn part2(input: &str) -> u64 {
    let input = input.replace('\n', "");
    let mut boxes: Vec<VecMap<String, u64>> = vec![VecMap::default(); 256];
    for s in input.split(',') {
        if s.contains('-') {
            let code = s.replace('-', "");
            let hash = hash(&code) as usize;
            let b = &mut boxes[hash];
            b.remove(&code);
        } else if s.contains('=') {
            let (name, power) = s.split_once('=').unwrap();
            let power: u64 = power.parse().unwrap();
            let name = name.to_string();
            let hash = hash(&name) as usize;
            let b = &mut boxes[hash];
            b.insert(name, power);
        }
    }
    let mut total = 0;
    for (i, b) in boxes.iter().enumerate() {
        let i = i as u64;
        let a = i + 1;
        for (j, v) in b.values().enumerate() {
            let j = j as u64;
            let b = j + 1;
            let c = v;
            total += a * b * c;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.trim()), 1320)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 145)
    }
}
