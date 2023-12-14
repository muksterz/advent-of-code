
use runner::aoc;

fn check_unique(input: &[char]) -> bool {
    for i in 0..input.len() {
        for j in (i..input.len()).skip(1) {
            if input[i] == input[j] {
                return false;
            }
        }
    }
    true
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    let mut iter = input.chars().enumerate();
    let (_, _1) = iter.next().unwrap();
    let (_, _2) = iter.next().unwrap();
    let (_, _3) = iter.next().unwrap();
    let _4 = _1;

    let mut buffer = [_1, _2, _3, _4];

    for(i, c) in iter {
        let loc = i % 4;
        buffer[loc] = c;

        if check_unique(&buffer) {
            return i as u64 + 1;
        }
    }

    unreachable!()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {

    const LEN: usize = 14;
    let mut iter = input.chars().enumerate();
    
    let a = [0; LEN -1];
    let a = a.map(|_| iter.next().unwrap().1);

    let mut buffer = ['0'; LEN];
    for (i, c) in a.into_iter().enumerate() {
        buffer[i] = c;
    }
    buffer[LEN - 1] = buffer[0];
    for(i, c) in iter {
        let loc = i % buffer.len();
        buffer[loc] = c;

        if check_unique(&buffer) {
            return i as u64 + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(7, super::part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, super::part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, super::part1("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, super::part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, super::part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part2() {
        assert_eq!(19, super::part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, super::part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, super::part2("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, super::part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, super::part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}