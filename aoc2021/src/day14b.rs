use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;


fn parse_rules(raw_rules: Vec<&str>) -> HashMap<(char, char), char> {
    let mut rules = HashMap::new();
    for rule in raw_rules {
        let cs = rule.trim().chars().collect::<Vec<char>>();
        rules.insert((cs[0], cs[1]), cs[6]);
    }
    rules
}

fn parse_polymer(polymer: String) -> HashMap<(char, char), u64> {
    let mut pairs = HashMap::new();
    let polymer_chars = polymer.chars().collect::<Vec<char>>();
    for i in 0..polymer_chars.len()-1 {
        if let Some(x) = pairs.get_mut(&(polymer_chars[i], polymer_chars[i+1])) {
            *x += 1;
        } else {
            pairs.insert((polymer_chars[i], polymer_chars[i+1]), 1);
        }
    }

    pairs
}

fn polymerize(polymer: HashMap<(char, char), u64>, rules: &HashMap<(char, char), char>) -> HashMap<(char, char), u64> {
    let mut new_polymer = HashMap::new();

    for (pair, count) in polymer {
        let inserted_char = *rules.get(&pair).unwrap();
        // first pair
        if let Some(x) = new_polymer.get_mut(&(pair.0, inserted_char)) {
            *x += count;
        } else {
            new_polymer.insert((pair.0, inserted_char), count);
        }
        // second pair
        if let Some(x) = new_polymer.get_mut(&(inserted_char, pair.1)) {
            *x += count;
        } else {
            new_polymer.insert((inserted_char, pair.1), count);
        }
    }

    new_polymer
}

fn count_unique_chars(polymer: HashMap<(char, char), u64>) -> HashMap<char, u64> {
    let mut char_count = HashMap::new();
    for (pair, count) in polymer {
        // first
        if let Some(x) = char_count.get_mut(&pair.0) {
            *x += count;
        } else {
            char_count.insert(pair.0, count);
        }
        // first
        if let Some(x) = char_count.get_mut(&pair.1) {
            *x += count;
        } else {
            char_count.insert(pair.1, count);
        }
    }
    for (_, val) in char_count.iter_mut() {
        if *val % 2 == 0 {
            *val = *val / 2;
        } else {
            *val = (*val + 1) / 2;
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
    let mut polymer = parse_polymer(String::from(inputs.next().unwrap()));
    let rules = parse_rules(inputs.next().unwrap().trim().split("\n").collect());

    let mut now = Instant::now();
    for round in 0..40 {
        polymer = polymerize(polymer, &rules);
        println!("Completed round {} in {} seconds", round, now.elapsed().as_secs_f32());
        now = Instant::now();
    }

    let char_counts = count_unique_chars(polymer);
    let mut most: u64 = 0;
    let mut least: u64 = u64::MAX;
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
