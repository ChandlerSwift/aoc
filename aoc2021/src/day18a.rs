use std::env;
use std::fs;
use std::fmt;
use std::ops::Add;
use std::iter::Sum;

#[derive(Eq, Debug, Clone)]
struct SnailfishNumber {
    regular_number: Option<u32>,
    pair_left: Option<Box<SnailfishNumber>>,
    pair_right: Option<Box<SnailfishNumber>>,
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.regular_number.is_some() {
            write!(f, "{}", self.regular_number.unwrap())
        } else {
            write!(f, "[{},{}]", self.pair_left.as_ref().unwrap(), self.pair_right.as_ref().unwrap()) // Why do I need as_ref() here?
        }
    }
}

impl PartialEq for SnailfishNumber {
    fn eq(&self, other: &Self) -> bool {
        self.regular_number == other.regular_number && self.pair_left == other.pair_left && self.pair_right == other.pair_right
    }
}

impl SnailfishNumber {
    fn from_string(string: &String) -> Self {
        if string.trim().chars().next().unwrap() != '[' { // Single number
            return SnailfishNumber{
                regular_number: Some(string.trim().parse().unwrap()),
                pair_left: None,
                pair_right: None,
            }
        }
        let mut chars = string.trim().chars();
        assert_eq!(chars.next().unwrap(), '[');
        // TODO: I should be able to do this with slices, rather than copying all the data
        let mut left = String::new(); 
        let mut right = String::new();
        let mut depth: u32 = 0;
        let mut second_pass = false; // Borrow checker workaround, TODO: avoid
        for c in chars {
            if c == ',' && depth == 0 {
                second_pass = true;
                continue;
            } else if c == '[' {
                depth += 1;
            } else if c == ']' {
                if depth > 0 {
                    depth -= 1;
                } else { // We're done
                    break;
                }
            }
            if second_pass {
                right.push(c);
            } else {
                left.push(c);
            }
        }
        SnailfishNumber{
            regular_number: None,
            pair_left: Some(Box::new(Self::from_string(&left))),
            pair_right: Some(Box::new(Self::from_string(&right))),
        }
    }

    fn reduce_with_depth<'a>(&'a mut self, depth: u32) -> bool {
        // If any pair is nested inside four pairs, the leftmost such pair
        // explodes.
        if depth >= 3 {
            println!("depth >= 3");
            if self.pair_left.is_some() && self.pair_left.as_ref().unwrap().pair_left.is_some() {
                println!("Exploding pair {}", self);
                // the left item of this pair is also a pair, so it needs to be exploded
                let exploding_pair = *self.pair_left.as_ref().unwrap().clone();
                // > Exploding pairs will always consist of two regular numbers.
                //
                // > To explode a pair, the pair's left value is added to the
                // > first regular number to the left of the exploding pair (if
                // > any)...
                // There will be none, because this number is on the left of its
                // pair.
                // > ...and the pair's right value is added to the first regular
                // > number to the right of the exploding pair (if any).
                let exploding_fragment = *exploding_pair.pair_right.unwrap();
                let explodes_onto = *self.pair_right.as_ref().unwrap().clone();
                self.pair_right = Some(Box::new(SnailfishNumber{
                    regular_number: Some(exploding_fragment.regular_number.unwrap() + explodes_onto.regular_number.unwrap()),
                    pair_left: None,
                    pair_right: None,
                }));
                // > Then, the entire exploding pair is replaced with the
                // > regular number 0.
                self.pair_left = Some(Box::new(SnailfishNumber{
                    regular_number: Some(0),
                    pair_left: None,
                    pair_right: None,
                }));
                println!("Exploded pair {}", self);
                return true;
            } else if self.pair_right.is_some() && self.pair_right.as_ref().unwrap().pair_left.is_some() {
                println!("Exploding pair {}", self);
                // as above
                let exploding_pair = *self.pair_right.as_ref().unwrap().clone();
                let exploding_fragment = *exploding_pair.pair_left.unwrap();
                let explodes_onto = *self.pair_left.as_ref().unwrap().clone();
                self.pair_left = Some(Box::new(SnailfishNumber{
                    regular_number: Some(exploding_fragment.regular_number.unwrap() + explodes_onto.regular_number.unwrap()),
                    pair_left: None,
                    pair_right: None,
                }));
                // > Then, the entire exploding pair is replaced with the
                // > regular number 0.
                self.pair_right = Some(Box::new(SnailfishNumber{
                    regular_number: Some(0),
                    pair_left: None,
                    pair_right: None,
                }));
                println!("Exploded pair {}", self);
                return true;
            }
        }
        if self.pair_left.is_some() {
            let to_reduce = self.pair_left.as_mut().unwrap();
            if to_reduce.reduce_with_depth(depth + 1) {
                return true;
            }
        }
        if self.pair_right.is_some() {
            let to_reduce = self.pair_right.as_mut().unwrap();
            if to_reduce.reduce_with_depth(depth + 1) {
                return true;
            }
        }

        // > To split a regular number, replace it with a pair; the left element
        // > of the pair should be the regular number divided by two and rounded
        // > down, while the right element of the pair should be the regular
        // > number divided by two and rounded up. For example, 10 becomes
        // > [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.
        if self.regular_number.is_some() && self.regular_number.unwrap() > 9 {
            self.pair_left = Some(Box::new(SnailfishNumber{
                regular_number: Some(self.regular_number.unwrap() / 2),
                pair_left: None,
                pair_right: None,
            }));
            self.pair_right = Some(Box::new(SnailfishNumber{
                regular_number: Some((self.regular_number.unwrap() + 1) / 2),
                pair_left: None,
                pair_right: None,
            }));
            self.regular_number = None;
            return true;
        }

        false
    }

    fn reduce(&mut self, ) {
        while self.reduce_with_depth(0) { }
    }

    fn magnitude(&self) -> u32 {
        self.regular_number.unwrap_or(
            3 * self.pair_left.as_ref().unwrap().magnitude() + 2 * self.pair_right.as_ref().unwrap().magnitude()
        )
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sn = SnailfishNumber{
            regular_number: None,
            pair_left: Some(Box::new(self)),
            pair_right: Some(Box::new(other)),
        };
        sn.reduce();
        sn
    }
}

// https://stackoverflow.com/a/28005283
impl<'a, 'b> Add<&'b SnailfishNumber> for &'a SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, other: &'b SnailfishNumber) -> SnailfishNumber {
        let mut sn = SnailfishNumber{
            regular_number: None,
            pair_left: Some(Box::new(self.clone())),
            pair_right: Some(Box::new(other.clone())),
        };
        sn.reduce();
        sn
    }
}

impl<'a> Add<&'a SnailfishNumber> for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, other: &'a SnailfishNumber) -> SnailfishNumber {
        let mut sn = SnailfishNumber{
            regular_number: None,
            pair_left: Some(Box::new(self)),
            pair_right: Some(Box::new(other.clone())),
        };
        sn.reduce();
        sn
    }
}

// TODO: I can't figure out how to make the borrow checker happy here
// https://users.rust-lang.org/t/implementing-the-sum-trait/23332/4
impl<'a> Sum<&'a Self> for SnailfishNumber {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        let mut sum = iter.next().unwrap().clone();
        for sn in iter {
            sum = sum + sn;
        }
        sum
        // or:
        // iter.reduce(|a,b| a+b).unwrap().clone()
    }
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
    let mut numbers = Vec::new();
    for line in raw_input.trim().split("\n") {
        numbers.push(SnailfishNumber::from_string(&String::from(line.trim())));
    }

    let sum: SnailfishNumber = numbers.iter().sum();
    println!("{}", sum.magnitude());
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        assert_eq!(SnailfishNumber{
                regular_number: Some(1),
                pair_left: None,
                pair_right: None
            },
            SnailfishNumber{
                regular_number: Some(1),
                pair_left: None,
                pair_right: None
            });
    }

    #[test]
    fn test_equality_nested() {
        assert_eq!(SnailfishNumber{
                regular_number: None,
                pair_left: Some(Box::new(SnailfishNumber{
                    regular_number: None,
                    pair_left: Some(Box::new(SnailfishNumber{
                        regular_number: Some(1),
                        pair_left: None,
                        pair_right: None
                    })),
                    pair_right: Some(Box::new(SnailfishNumber{
                        regular_number: Some(1),
                        pair_left: None,
                        pair_right: None
                    }))
                })),
                pair_right: Some(Box::new(SnailfishNumber{
                    regular_number: Some(1),
                    pair_left: None,
                    pair_right: None
                }))
            },
            SnailfishNumber{
                regular_number: None,
                pair_left: Some(Box::new(SnailfishNumber{
                    regular_number: None,
                    pair_left: Some(Box::new(SnailfishNumber{
                        regular_number: Some(1),
                        pair_left: None,
                        pair_right: None
                    })),
                    pair_right: Some(Box::new(SnailfishNumber{
                        regular_number: Some(1),
                        pair_left: None,
                        pair_right: None
                    }))
                })),
                pair_right: Some(Box::new(SnailfishNumber{
                    regular_number: Some(1),
                    pair_left: None,
                    pair_right: None
                }))
            });
    }

    #[test]
    fn test_parsing() {
        assert_eq!(SnailfishNumber::from_string(&String::from("[1,2]")), SnailfishNumber{
            regular_number: None,
            pair_left: Some(Box::new(SnailfishNumber{
                regular_number: Some(1),
                pair_left: None,
                pair_right: None,
            })),
            pair_right: Some(Box::new(SnailfishNumber{
                regular_number: Some(2),
                pair_left: None,
                pair_right: None,
            })),
        });
    }

    #[test]
    fn test_parsing_and_formatting() {
        for example in "[1,2]
        [[1,2],3]
        [9,[8,7]]
        [[1,9],[8,5]]
        [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
        [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
        [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]".split("\n") {
            let example = example.trim();
            let sn = SnailfishNumber::from_string(&String::from(example));
            assert_eq!(example, sn.to_string());
        }
    }

    #[test]
    fn test_split() {
        let mut sn = SnailfishNumber::from_string(&String::from("10"));
        sn.reduce();
        assert_eq!(sn.to_string(), "[5,5]");

        sn = SnailfishNumber::from_string(&String::from("11"));
        sn.reduce();
        assert_eq!(sn.to_string(), "[5,6]");

        sn = SnailfishNumber::from_string(&String::from("12"));
        sn.reduce();
        assert_eq!(sn.to_string(), "[6,6]");
    }

    #[test]
    fn test_explode() {
        let examples = vec![
            "[[[[[9,8],1],2],3],4]",
            "[7,[6,[5,[4,[3,2]]]]]",
            "[[6,[5,[4,[3,2]]]],1]",
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", // two actions
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ];
        let solutions = vec![
            "[[[[0,9],2],3],4]",
            "[7,[6,[5,[7,0]]]]",
            "[[6,[5,[7,0]]],3]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ];

        for i in 0..examples.len() {
            let mut sn = SnailfishNumber::from_string(&String::from(examples[i]));
            sn.reduce();
            assert_eq!(sn.to_string(), solutions[i]);
        }
    }

}
