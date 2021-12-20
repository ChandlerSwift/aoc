use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn parse_rules(raw_rules: Vec<&str>) -> Vec<((char, char), char)> {
    let mut rules = Vec::new();
    for rule in raw_rules {
        let cs = rule.trim().chars().collect::<Vec<char>>();
        rules.push(((cs[0], cs[1]), cs[6]));
    }
    rules
}

fn polymerize(polymer_base: String, rules: &Vec<((char, char), char)>) -> String {
    let mut new_polymer = String::new();
    let polymer_chars = polymer_base.chars().collect::<Vec<char>>();
    for i in 0..polymer_chars.len()-1 {
        new_polymer.push(polymer_chars[i]);
        for rule in rules {
            // println!("Comparing {},{} and {},{}", polymer_chars[i], rule.0.0,  polymer_chars[i+1],  rule.0.1);
            if polymer_chars[i] == rule.0.0 && polymer_chars[i+1] == rule.0.1 {
                new_polymer.push(rule.1);
            }
        }
    }
    new_polymer.push(polymer_chars[polymer_chars.len()-1]);
    new_polymer
}

fn count_unique_chars(polymer: String) -> HashMap<char, u32> {
    let mut char_count = HashMap::new();
    for c in polymer.chars() {
        if let Some(x) = char_count.get_mut(&c) {
            *x += 1;
        } else {
            char_count.insert(c, 1);
        }
    }
    char_count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];
    let raw_input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inputs = raw_input.trim().split("\n\n");
    let mut polymer = String::from(inputs.next().unwrap());
    let rules = parse_rules(inputs.next().unwrap().trim().split("\n").collect());

    let mut now = Instant::now();
    for round in 0..10 {
        polymer = polymerize(polymer, &rules);
        println!("Completed round {} in {} seconds", round, now.elapsed().as_secs_f32());
        now = Instant::now();
    }

    let char_counts = count_unique_chars(polymer);
    let mut most = 0;
    let mut least = 1_000_000_000;
    for (_char, count) in char_counts {
        if count > most {
            most = count;
        }
        if count < least {
            least = count;
        }
    }
    println!("{}", most - least);
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    #[test]
    fn test_polymerize() {
        let mut inputs = INPUT.trim().split("\n\n");
        let mut polymer = String::from(inputs.next().unwrap());
        let rules = parse_rules(inputs.next().unwrap().trim().split("\n").collect());

        polymer = polymerize(polymer, &rules);
        assert_eq!(polymer, "NCNBCHB");

        polymer = polymerize(polymer, &rules);
        assert_eq!(polymer, "NBCCNBBBCBHCB");

        polymer = polymerize(polymer, &rules);
        assert_eq!(polymer, "NBBBCNCCNBBNBNBBCHBHHBCHB");

        polymer = polymerize(polymer, &rules);
        assert_eq!(polymer, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");

        polymer = polymerize(polymer, &rules);
        assert_eq!(polymer.len(), 97);

        polymer = polymerize(polymer, &rules);
        polymer = polymerize(polymer, &rules);
        polymer = polymerize(polymer, &rules);
        polymer = polymerize(polymer, &rules);
        polymer = polymerize(polymer, &rules);
        assert_eq!(polymer.len(), 3073);

        let char_counts = count_unique_chars(polymer);
        let mut most = 0;
        let mut least = 1_000_000_000;
        for (_char, count) in char_counts {
            if count > most {
                most = count;
            }
            if count < least {
                least = count;
            }
        }
        assert_eq!(most, 1749);
        assert_eq!(least, 161);
    }

}
