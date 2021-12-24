use std::env;
use std::fs;

fn identify_digits(_combinations: Vec<&str>, currently_displayed: Vec<&str>) -> String {
    let mut ret = String::new();

    for digit in currently_displayed {
        match digit.len() {
            2 => ret.push('1'),
            3 => ret.push('7'),
            4 => ret.push('4'),
            7 => ret.push('8'),
            _ => ret.push('x'),
        }
    }
    return ret;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];
    let raw_inputs = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut ones_fours_sevens_eights_count = 0;
    for input in raw_inputs.trim().split("\n") {
        let input_sections = input.split(" | ").collect::<Vec<&str>>();
        let patterns = input_sections[0].trim().split(" ").collect::<Vec<&str>>();
        assert_eq!(patterns.len(), 10);
        let output_digits = input_sections[1].split(" ").collect::<Vec<&str>>();
        let correct_digits = identify_digits(patterns, output_digits);
        for digit in correct_digits.chars() {
            if digit == '1' || digit == '4' || digit == '7' || digit == '8' { // TODO: is there a better way to do this?
                ones_fours_sevens_eights_count += 1;
            }
        }
    }
    println!("{}", ones_fours_sevens_eights_count);
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str ="
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    ";

    #[test]
    fn test_count_1478_digit_occurrences() {
        let mut ones_fours_sevens_eights_count = 0;
        for input in INPUT.trim().split("\n") {
            let input_sections = input.split(" | ").collect::<Vec<&str>>();
            let patterns = input_sections[0].trim().split(" ").collect::<Vec<&str>>();
            assert_eq!(patterns.len(), 10);
            let output_digits = input_sections[1].split(" ").collect::<Vec<&str>>();
            let correct_digits = identify_digits(patterns, output_digits);
            for digit in correct_digits.chars() {
                if digit == '1' || digit == '4' || digit == '7' || digit == '8' { // TODO: is there a better way to do this?
                    ones_fours_sevens_eights_count += 1;
                }
            }
        }
        assert_eq!(ones_fours_sevens_eights_count, 26);
    }
}
