use std::collections::HashMap;

use runner::aoc;

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default: String, // Workflow name
}

type Workflows = HashMap<String, Workflow>;

impl Workflow {
    fn process(&self, obj: &Object) -> &str {
        for f in self.rules.iter() {
            if let Some(s) = f.matches(obj) {
                return s;
            }
        }
        &self.default
    }


}

#[derive(Clone, Debug)]
struct Rule {
    var: char,
    cond: Cond,
    val: i64,
    next_flow: String,
}

impl Rule {
    fn matches(&self, input: &Object) -> Option<&str> {
        let v = input.get_var(self.var);
        let cond = match self.cond {
            Cond::Gt => v > self.val,
            Cond::Lt => v < self.val,
        };

        if cond {
            Some(&self.next_flow)
        } else {
            None
        }
    }


}

#[derive(Clone, Copy, Debug)]
enum Cond {
    Gt,
    Lt,
}

struct Object {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Object {
    fn get_var(&self, v: char) -> i64 {
        match v {
            'a' => self.a,
            'x' => self.x,
            'm' => self.m,
            's' => self.s,
            _ => panic!(),
        }
    }
}

fn parse_workflows(input: &str) -> HashMap<String, Workflow> {
    let mut workflows = HashMap::new();

    for l in input.trim().lines().map(str::trim) {
        let (name, rest) = l.split_once('{').unwrap();
        let rest = rest.trim_end_matches('}');

        let mut rules = Vec::new();
        let last;

        let mut iter = rest.split(',');

        loop {
            let v = iter.next().unwrap();

            if !v.contains(':') {
                last = v.to_string();
                break;
            }

            let (cond, next) = v.split_once(':').unwrap();
            let rule = if cond.contains('<') {
                let (name, val) = cond.split_once('<').unwrap();
                Rule {
                    var: name.chars().next().unwrap(),
                    cond: Cond::Lt,
                    val: val.parse().unwrap(),
                    next_flow: next.into(),
                }
            } else {
                let (name, val) = cond.split_once('>').unwrap();
                Rule {
                    var: name.chars().next().unwrap(),
                    cond: Cond::Gt,
                    val: val.parse().unwrap(),
                    next_flow: next.into(),
                }
            };

            rules.push(rule)
        }

        workflows.insert(
            name.into(),
            Workflow {
                rules,
                default: last,
            },
        );
    }

    workflows
}

fn parse_objects(input: &str) -> Vec<Object> {
    let mut out = Vec::new();
    for l in input.trim().lines().map(str::trim) {
        let l = l.trim_matches('}').trim_matches('{');
        let mut iter = l.split(',');
        let x = iter.next().unwrap();
        let m = iter.next().unwrap();
        let a = iter.next().unwrap();
        let s = iter.next().unwrap();

        let x = x.split_once('=').unwrap().1.parse().unwrap();
        let m = m.split_once('=').unwrap().1.parse().unwrap();
        let a = a.split_once('=').unwrap().1.parse().unwrap();
        let s = s.split_once('=').unwrap().1.parse().unwrap();

        out.push(Object { x, m, a, s });
    }
    out
}

#[aoc(day19, part1)]
fn part1(input: &str) -> i64 {
    let (workflows, objects) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);
    let objects = parse_objects(objects);

    let mut total = 0;

    for o in objects.iter() {
        let mut process = "in";

        while process != "A" && process != "R" {
            process = workflows[process].process(o);
        }

        if process == "A" {
            total += o.x + o.m + o.a + o.s;
        }
    }

    total
}

#[derive(Debug, Clone)]
struct ObjectSet {
    x: RangeSet,
    m: RangeSet,
    a: RangeSet,
    s: RangeSet,
}

impl ObjectSet {
    fn full() -> Self {
        Self {
            x: RangeSet::full(),
            m: RangeSet::full(),
            a: RangeSet::full(),
            s: RangeSet::full(),
        }
    }

    fn count(&self) -> i64 {
        self.x.values() * self.m.values() * self.a.values() * self.s.values()
    }

}

#[derive(Clone, Debug)]
struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    fn full() -> Self {
        RangeSet {
            ranges: vec![Range::full()],
        }
    }

    fn empty() -> Self {
        RangeSet { ranges: Vec::new() }
    }

    fn union(&mut self, range: Range) {
        let start = self
            .ranges
            .iter()
            .filter(|r| r.contains(range.start))
            .map(|r| r.start)
            .next()
            .unwrap_or(range.start);
        let end = self
            .ranges
            .iter()
            .filter(|r| r.contains(range.end))
            .map(|r| r.end)
            .last()
            .unwrap_or(range.end);

        let range = Range { start, end };
        self.ranges.retain(|r| !range.contains_range(*r));

        self.ranges.push(range);

        self.ranges.sort();
    }

    fn intersect(&mut self, range: Range) {
        let ranges = self
            .ranges
            .drain(..)
            .flat_map(|r| r.intersection(range))
            .collect();
        *self = RangeSet { ranges }
    }

    fn union_set(&mut self, set: &RangeSet) {
        for r in set.ranges.iter() {
            self.union(*r);
        }
    }

    fn intersect_set(&mut self, set: &RangeSet) {
        let mut new_ranges = RangeSet::empty();

        for r in self.ranges.iter().copied() {
            let mut new_set = set.clone();
            new_set.intersect(r);
            new_ranges.union_set(&new_set);
        }

        *self = new_ranges
    }

    fn invert(&mut self) {
        if self.ranges.is_empty() {
            *self = RangeSet::full();
            return;
        }

        let mut inverted = Vec::new();

        if self.ranges[0].start != 0 {
            inverted.push(Range::lt(self.ranges[0].start));
        }

        for w in self.ranges.windows(2) {
            let r1 = w[0];
            let r2 = w[1];
            inverted.push(Range {
                start: r1.end + 1,
                end: r2.start - 1,
            });
        }

        if self.ranges.last().unwrap().end != 4000 {
            inverted.push(Range::gt(self.ranges.last().unwrap().end))
        }

        self.ranges = inverted;
    }

    fn values(&self) -> i64 {
        self.ranges.iter().map(|r| r.end - r.start + 1).sum()
    }

    fn subtract_range(&mut self, range: Range) {
        let mut set = RangeSet::empty();
        set.union(range);
        self.subtract(&set)
    }

    fn subtract(&mut self, other: &RangeSet) {
        let mut other = other.clone();
        other.invert();
        self.intersect_set(&other)
    }
}

// same as start..=end
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range {
    start: i64,
    end: i64,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.end < other.start {
            Some(std::cmp::Ordering::Less)
        } else if other.end < self.start {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}
impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Range {
    fn full() -> Self {
        Range {
            start: 1,
            end: 4000,
        }
    }

    fn gt(v: i64) -> Self {
        Self {
            start: v + 1,
            end: 4000,
        }
    }
    fn lt(v: i64) -> Self {
        Self {
            start: 0,
            end: v - 1,
        }
    }

    fn cond(cond: Cond, v: i64) -> Self {
        match cond {
            Cond::Gt => Self::gt(v),
            Cond::Lt => Self::lt(v),
        }
    }
    fn contains(&self, v: i64) -> bool {
        self.start <= v && v <= self.end
    }

    fn contains_range(&self, r: Range) -> bool {
        self.start <= r.start && r.end <= self.end
    }

    fn intersection(self, r: Range) -> Option<Range> {
        if r.contains_range(self) {
            return Some(self);
        }

        match (self.contains(r.start), self.contains(r.end)) {
            (true, true) => Some(r),
            (true, false) => Some(Range {
                start: r.start,
                end: self.end,
            }),
            (false, true) => Some(Range {
                start: self.start,
                end: r.end,
            }),
            (false, false) => None,
        }
    }
}

fn vals_rec(workflows: &Workflows, flow: &str, mut default: ObjectSet) -> i64 {
    if flow == "A" {
        return default.count()
    } else if flow == "R" {
        return 0
    }

    let workflow = &workflows[flow];

    let mut total = 0;

    for rule in workflow.rules.iter() {
        let range = Range::cond(rule.cond, rule.val);

        let mut next = default.clone();

        match rule.var {
            'x' => {
                default.x.subtract_range(range);
                next.x.intersect(range)
            },
            'm' => {
                default.m.subtract_range(range);
                next.m.intersect(range)
            },
            'a' => {
                default.a.subtract_range(range);
                next.a.intersect(range)
            },
            's' => {
                default.s.subtract_range(range);
                next.s.intersect(range)
            },
            _ => panic!()
        }


        total += vals_rec(workflows, &rule.next_flow, next);
    }

    total + vals_rec(workflows, &workflow.default, default)

}

fn workflow_vals(workflows: &Workflows, flow: &str) -> i64 {
    let obj = ObjectSet::full();
    vals_rec(workflows, flow, obj)
}

#[aoc(day19, part2)]
fn part2(input: &str) -> i64 {
    let workflows = input.split_once("\n\n").unwrap().0;
    let workflows = parse_workflows(workflows);

    // gd{a>3333:R,R}
    // rfg{s<537:gd,x>2440:R,A}
    // crn{x>2662:A,R}

    workflow_vals(&workflows, "in")
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    ";

    #[test]
    fn test_internals() {
        let workflows = parse_workflows(INPUT.split("\n\n").next().unwrap());

        
        assert_eq!(workflow_vals(&workflows, "crn"), (4000-2662)*4000*4000*4000);
        assert_eq!(workflow_vals(&workflows, "gd"), 0);


    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 19114)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 167409079868000)
    }


}
