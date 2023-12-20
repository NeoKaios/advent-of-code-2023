use std::{fs, env, collections::HashMap};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content);
}

fn compute(content: &String) {
    let split = content.find("\n\n").expect("No empty line");
    let re_ruleset = Regex::new(r"^([a-z]+)\{((?:[xmas][<>]\d+:(?:A|R|[a-z]+),?)+),(A|R|[a-z]+)\}").unwrap();
    let re_rules = Regex::new(r"([xmas])([<>])(\d+):(A|R|[a-z]+)").unwrap();
    let re_parts = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    let mut all_rules = HashMap::new();
    for line in content[..split].lines() {
        let (id, ruleset) = RuleSet::parse(line, &re_ruleset, &re_rules);
        all_rules.insert(id, ruleset);
    }
    let sum = content[split+2..].lines().fold(0, |acc,line| {
        let part = Part::parse(line, &re_parts);
        let mut current_rule_id = "in";
        while let Some(ruleset) = all_rules.get(current_rule_id) {
            match ruleset.find_dest(&part) {
                Dest::R => return acc,
                Dest::A => break,
                Dest::Ruleset(id) => current_rule_id = id,
            }
        }
        acc + part.get_combined_ratings()
    });
    println!("The sum of the accepted part's rating is: {sum}");
    let res = count_accepted_parts(&all_rules, PartSet::new_maximized_partset(), "in");
    println!("The number of acceptable parts is: {res}");
}

fn count_accepted_parts(all_rules: &HashMap<&str, RuleSet<'_>>, mut partset: PartSet, current_ruleset_id: &str) -> u64 {
    let Some(ruleset) = all_rules.get(current_ruleset_id) else {return 0;};
    let mut accepted_parts_count = 0;
    for rule in &ruleset.rules {
        // Reduce the partset so it pass the rule and send to dest
        if let Some(reduced_partset) = rule.reduce(&partset) {
            accepted_parts_count += match rule.dest {
                Dest::R => 0,
                Dest::A => reduced_partset.count(),
                Dest::Ruleset(id) => count_accepted_parts(all_rules, reduced_partset, id),
            };
        }
        // also reduce partset to inverse of the rule and continue in ruleset
        let Some(reduced_partset) = rule.inv().reduce(&partset) else {return accepted_parts_count;};
        partset = reduced_partset;
    }
    accepted_parts_count + match ruleset.default {
        Dest::R => 0,
        Dest::A => partset.count(),
        Dest::Ruleset(id) => count_accepted_parts(all_rules, partset, id),
    }
}

#[derive(Debug, Clone)]
struct PartSet {
    x_lower_bound: usize,
    x_upper_bound: usize,
    m_lower_bound: usize,
    m_upper_bound: usize,
    a_lower_bound: usize,
    a_upper_bound: usize,
    s_lower_bound: usize,
    s_upper_bound: usize,
}

impl PartSet {
    fn new_maximized_partset() -> PartSet {
        PartSet {
            x_lower_bound: 0, x_upper_bound: 4001,
            m_lower_bound: 0, m_upper_bound: 4001,
            a_lower_bound: 0, a_upper_bound: 4001,
            s_lower_bound: 0, s_upper_bound: 4001,
        }
    }
    /// Count the parts in the set
    fn count(&self) -> u64 {
        (self.x_upper_bound - self.x_lower_bound - 1) as u64 *
        (self.m_upper_bound - self.m_lower_bound - 1) as u64 *
        (self.a_upper_bound - self.a_lower_bound - 1) as u64 *
        (self.s_upper_bound - self.s_lower_bound - 1) as u64
    }
    /// Return a tuple of the lower and upper bound for a given category
    fn get(&self, cat: Cat) -> (usize, usize) {
        match cat {
            Cat::X => (self.x_lower_bound, self.x_upper_bound),
            Cat::M => (self.m_lower_bound, self.m_upper_bound),
            Cat::A => (self.a_lower_bound, self.a_upper_bound),
            Cat::S => (self.s_lower_bound, self.s_upper_bound),
        }
    }
    /// Update the set with a new lower and upper bound for a given category
    fn update(&mut self, cat: Cat, lower_bound: usize, upper_bound: usize) {
        let (cat_lower_bound,cat_upper_bound) = match cat {
            Cat::X => (&mut self.x_lower_bound, &mut self.x_upper_bound),
            Cat::M => (&mut self.m_lower_bound, &mut self.m_upper_bound),
            Cat::A => (&mut self.a_lower_bound, &mut self.a_upper_bound),
            Cat::S => (&mut self.s_lower_bound, &mut self.s_upper_bound),
        };
        *cat_lower_bound = lower_bound;
        *cat_upper_bound = upper_bound;
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(s: &str, re_parts: &Regex) -> Part {
        let [x,m,a,s] = re_parts.captures(s).expect("Should match part").extract::<4>().1.map(|s| s.parse::<usize>().expect("digit"));
        Part {x,m,a,s}
    }
    /// Get the sum of the part's rating
    fn get_combined_ratings(&self) -> i32 {
        (self.x + self.m + self.a + self.s) as i32
    }
    /// Get the category rating of the part
    fn get(&self, cat: Cat) -> usize {
        match cat {
            Cat::X => self.x,
            Cat::M => self.m,
            Cat::A => self.a,
            Cat::S => self.s,
        }
    }
}

#[derive(Debug)]
struct RuleSet<'a> {
    rules: Vec<Rule<'a>>,
    default: Dest<'a>,
}

impl <'a> RuleSet<'a> {
    fn parse(s: &'a str, re_ruleset: &Regex, re_rules: &Regex) -> (&'a str, RuleSet<'a>) {
        let mut rules = vec!();
        let [id, rules_str, default] = re_ruleset.captures(s).expect("Ruleset should match").extract::<3>().1;
        Rule::parse(rules_str, &re_rules, &mut rules);
        (id, RuleSet { default: Dest::parse(default), rules })
    }
    /// Iterate over the rule in the ruleset to find the correct
    /// [Dest] for a given part
    fn find_dest(&self, part: &Part) -> Dest<'a> {
        for rule in self.rules.iter() {
            if rule.is_match(part) {
                return rule.dest;
            }
        }
        self.default
    }
}

#[derive(Debug)]
struct Rule<'a> {
    cat: Cat,
    greater: bool,
    criteria: usize,
    dest: Dest<'a>
}

impl <'a> Rule<'a> {
    fn parse(s: &'a str, re: &Regex, out: &mut Vec<Rule<'a>>){
        for (_, [cat, greater, digit, dest]) in re.captures_iter(s).map(|caps| caps.extract()) {
            out.push( Rule {
                cat: Cat::parse(cat),
                greater: greater == ">",
                criteria: digit.parse().expect(format!("Found unparsable digits {digit}").as_str()),
                dest: Dest::parse(dest)
            });
        }
    }
    /// Check if a part respects the rule
    fn is_match(&self, part: &Part) -> bool {
        let value = part.get(self.cat);
        self.greater && value > self.criteria || !self.greater && value < self.criteria
    }

    /// Reduce the partset so that every parts in the set match the rule
    ///
    /// If no parts are left in the set, return [None]
    ///
    /// Otherwise return a reduced partset
    fn reduce(&self, partset: &PartSet) -> Option<PartSet> {

        let (mut lower_bound, mut upper_bound) = partset.get(self.cat);
        if self.greater {
            lower_bound = std::cmp::max(lower_bound, self.criteria);
        } else {
            upper_bound = std::cmp::min(upper_bound, self.criteria);
        }
        if lower_bound + 1 < upper_bound {
            let mut reduced_partset = partset.clone();
            reduced_partset.update(self.cat, lower_bound, upper_bound);
            Some(reduced_partset)
        } else {
            None
        }
    }
    /// Create an inversed copy of the rule
    ///
    /// That copy match all the parts the origina rule rejected
    fn inv(&self) -> Rule {
        Rule {
            cat: self.cat,
            criteria: if self.greater {self.criteria + 1} else {self.criteria-1},
            greater: !self.greater,
            dest: self.dest
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Dest<'a> {
    R,A,
    Ruleset(&'a str)
}

impl <'a> Dest<'a> {
    fn parse(s: &str) -> Dest {
        match s {
            "R" => Dest::R,
            "A" => Dest::A,
            _ => Dest::Ruleset(s),

        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cat {
    X,M,A,S
}

impl Cat {
    fn parse(s: &str) -> Cat {
        match s {
            "x" => Cat::X,
            "m" => Cat::M,
            "a" => Cat::A,
            "s" => Cat::S,
            _ => panic!("Not a possible category"),
        }
    }
}
