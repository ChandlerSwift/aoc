use std::env;
use std::fs;
use std::collections::HashMap;

fn identify_digits(combinations: Vec<&str>, currently_displayed: Vec<&str>) -> String {
    // Segments on the display occur in some fraction of the ten decimal digits
    // that are displayed. The bottom left segment, represented by "e" below:
    //
    //  aaaa
    // b    c
    // b    c
    //  dddd
    // e    f
    // e    f
    //  gggg
    // -----
    // |   |
    // |---|
    // X   |
    // -----
    // occurs four times, in the zero, two, six, and eight digits. The bottom
    // left digit, represented by "f", occurs in each digit but two. Not all
    // are distinct and can be identified solely by this method.
    //
    //  digit | times occurred
    // -------+----------------
    //    e   |       4
    //    b   |       6
    //  d, g  |       7
    //  a, c  |       8
    //    f   |       9
    //
    // We begin by identifying the frequency each digit occurs:
    let mut segment_frequencies = HashMap::new();
    for combination in combinations {
        for c in combination.chars() {
            let segment = segment_frequencies.entry(c).or_insert(0);
            *segment += 1;
        }
    }

    // Then, for each digit that is currently being displayed:
    let mut ret = String::new();
    for digit in currently_displayed {
        // We create an array of the number of times each of its digits occurs
        // in the set of all unique digits.
        let mut digit_segment_frequencies = Vec::new();
        for c in digit.chars() {
            let segment_frequency = segment_frequencies.get(&c).unwrap();
            let segment_frequency = char::from_digit(*segment_frequency, 10).unwrap();
            digit_segment_frequencies.push(segment_frequency);
        }
        // This sequence is known to be unique. We order the digits, since they
        // don't come in in any known order, and convert that into a string to
        // make comparisons easier.
        digit_segment_frequencies.sort_unstable();
        let digit_segment_frequencies = String::from_iter(digit_segment_frequencies);
        ret.push(match digit_segment_frequencies.as_str() {
            "467889" => '0',
            "89" => '1',
            "47788" => '2',
            "77889" => '3',
            "6789" => '4',
            "67789" => '5',
            "467789" => '6',
            "889" => '7',
            "4677889" => '8',
            "677889" => '9',
            _ => panic!("No number matched"),
        })
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

    let mut total = 0;
    for input in raw_inputs.trim().split("\n") {
        let input_sections = input.split(" | ").collect::<Vec<&str>>();
        let patterns = input_sections[0].trim().split(" ").collect::<Vec<&str>>();
        assert_eq!(patterns.len(), 10);
        let output_digits = input_sections[1].split(" ").collect::<Vec<&str>>();
        let correct_digits = identify_digits(patterns, output_digits);
        let displayed_value: u32 = correct_digits.parse().unwrap();
        total += displayed_value;
    }
    println!("{}", total);
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
    fn test_count_displayed_digits_total() {
        let mut total = 0;
        for input in INPUT.trim().split("\n") {
            let input_sections = input.split(" | ").collect::<Vec<&str>>();
            let patterns = input_sections[0].trim().split(" ").collect::<Vec<&str>>();
            assert_eq!(patterns.len(), 10);
            let output_digits = input_sections[1].split(" ").collect::<Vec<&str>>();
            let correct_digits = identify_digits(patterns, output_digits);
            let displayed_value: u32 = correct_digits.parse().unwrap();
            total += displayed_value;
        }
        assert_eq!(total, 61229);
    }
}
