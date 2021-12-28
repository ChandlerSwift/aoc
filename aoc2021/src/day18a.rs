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

#[derive(Clone, PartialEq, Eq, Debug)]
enum TokenType {
    LeftBracket,
    RightBracket,
    Comma,
    Number
}

#[derive(Clone, Debug)]
struct Token {
    token_type: TokenType,
    value: Option<u32>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.token_type {
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Comma => write!(f, ","),
            TokenType::Number => write!(f, "{}", self.value.unwrap()),
        }
    }
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

    fn tokenize(&self) -> Vec<Token> {
        let mut vec = Vec::new();
        let self_str = self.to_string();
        let mut char_iter = self_str.chars().peekable(); // https://stackoverflow.com/a/62190290

        while let Some(c) = char_iter.next() {
            vec.push(match c {
                '[' => Token{
                    token_type: TokenType::LeftBracket,
                    value: None,
                },
                ']' => Token{
                    token_type: TokenType::RightBracket,
                    value: None,
                },
                ',' => Token{
                    token_type: TokenType::Comma,
                    value: None,
                },
                d => { // Anything else should be a number, possibly with multiple digits
                    let mut s = String::new();
                    s.push(d);
                    while char_iter.peek().is_some() && char_iter.peek().unwrap().is_ascii_digit() {
                        s.push(char_iter.next().unwrap());
                    }
                    Token{
                        token_type: TokenType::Number,
                        value: Some(s.parse().unwrap()),
                    }
                },
            });
        }
        vec
    }

    fn reduce(&mut self) {
        let mut reduced = true;
        while reduced {
            reduced = false;
            let tokens = self.tokenize();
            let mut new_tokens = Vec::new();
            let mut depth = 0;
            let mut token_iter = 0..tokens.len();
            while let Some(i) = token_iter.next() {
                match tokens[i].token_type {
                    TokenType::LeftBracket => {
                        if depth < 4 {
                            depth += 1;
                        } else if depth == 4 {
                            // boom!
                            reduced = true;
                            // Take the pair off the stack:
                            let left = tokens[token_iter.next().unwrap()].clone();
                            assert_eq!(left.token_type, TokenType::Number);
                            assert_eq!(tokens[token_iter.next().unwrap()].token_type, TokenType::Comma);
                            let mut right = tokens[token_iter.next().unwrap()].clone();
                            assert_eq!(right.token_type, TokenType::Number);
                            assert_eq!(tokens[token_iter.next().unwrap()].token_type, TokenType::RightBracket);
                            // replace it with a zero:
                            new_tokens.push(Token{
                                token_type: TokenType::Number,
                                value: Some(0),
                            });
                            // propagate them to the left:
                            for j in (0..i).rev() {
                                if tokens[j].token_type == TokenType::Number {
                                    let new_value = tokens[j].value.unwrap() + left.value.unwrap();
                                    new_tokens[j].value = Some(new_value);
                                    break;
                                }
                            }
                            // and right: -- this is kinda hacky; I could easily use the existing iterator! TODO
                            while let Some(j) = token_iter.next() { // This also copies over the rest of the tokens
                                if tokens[j].token_type == TokenType::Number {
                                    let mut new_token = tokens[j].clone();
                                    let new_value = new_token.value.unwrap() + right.value.unwrap();
                                    new_token.value = Some(new_value);
                                    new_tokens.push(new_token);
                                    right.value = Some(0);
                                } else {
                                    new_tokens.push(tokens[j].clone());
                                }
                            }
                            break;
                        } else {
                            panic!("Exceeded max expected depth");
                        }
                    },
                    TokenType::RightBracket => {
                        depth -= 1;
                    }
                    TokenType::Comma => {
                    },
                    TokenType::Number => {
                    },
                }
                new_tokens.push(tokens[i].clone());
            }
            assert_eq!(token_iter.next(), None);

            if !reduced { // We could also try splitting:
                let tokens = new_tokens;
                new_tokens = Vec::new();
                for token in tokens {
                    if token.value.is_some() && token.value.unwrap() > 9 && !reduced {
                        new_tokens.push(Token{token_type: TokenType::LeftBracket, value: None});
                        new_tokens.push(Token{token_type: TokenType::Number, value: Some(token.value.unwrap() / 2)});
                        new_tokens.push(Token{token_type: TokenType::Comma, value: None});
                        new_tokens.push(Token{token_type: TokenType::Number, value: Some((token.value.unwrap() + 1) / 2)});
                        new_tokens.push(Token{token_type: TokenType::RightBracket, value: None});
                        reduced = true;
                    } else {
                        new_tokens.push(token.clone());
                    }
                }
            }

            let mut new_str = String::new();
            for token in new_tokens {
                new_str.push_str(&token.to_string());
            }
            let new_self = SnailfishNumber::from_string(&new_str);
            self.regular_number = new_self.regular_number;
            self.pair_left = new_self.pair_left;
            self.pair_right = new_self.pair_right;

        }
    }

    fn magnitude(&self) -> u32 {
        if self.regular_number.is_some() {
            self.regular_number.unwrap()
        } else {
            3 * self.pair_left.as_ref().unwrap().magnitude() + 2 * self.pair_right.as_ref().unwrap().magnitude()
        }
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
            println!("Trying {}", examples[i]);
            let mut sn = SnailfishNumber::from_string(&String::from(examples[i]));
            sn.reduce();
            assert_eq!(sn.to_string(), solutions[i]);
        }
    }

    #[test]
    fn test_add() {
        let addend1 = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let addend2 = "[1,1]";
        let sn1 = SnailfishNumber::from_string(&String::from(addend1));
        let sn2 = SnailfishNumber::from_string(&String::from(addend2));
        let sum = sn1+sn2;
        assert_eq!(sum.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn test_add_all1() {
        let raw_numbers = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
        ];
        let mut numbers = Vec::new();
        for raw_number in raw_numbers {
            numbers.push(SnailfishNumber::from_string(&String::from(raw_number)));
        }
        let sum: SnailfishNumber = numbers.iter().sum();
        assert_eq!(sum.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn test_add_all2() {
        let raw_numbers = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
            "[5,5]",
        ];
        let mut numbers = Vec::new();
        for raw_number in raw_numbers {
            numbers.push(SnailfishNumber::from_string(&String::from(raw_number)));
        }
        let sum: SnailfishNumber = numbers.iter().sum();
        assert_eq!(sum.to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn test_add_all3() {
        let raw_numbers = vec![
            "[1,1]",
            "[2,2]",
            "[3,3]",
            "[4,4]",
            "[5,5]",
            "[6,6]",
        ];
        let mut numbers = Vec::new();
        for raw_number in raw_numbers {
            numbers.push(SnailfishNumber::from_string(&String::from(raw_number)));
        }
        let sum: SnailfishNumber = numbers.iter().sum();
        assert_eq!(sum.to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_add_all4() {
        let raw_numbers = "
        [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]
        ";


        let mut numbers = Vec::new();

        for line in raw_numbers.trim().split("\n") {
            numbers.push(SnailfishNumber::from_string(&String::from(line.trim())));
        }

        let sum: SnailfishNumber = numbers.iter().sum();
        assert_eq!(sum.to_string(), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn test_magnitude() {
        let inputs = vec![
            "[9,1]",
            "[1,9]",
            "[[9,1],[1,9]]",
            "[[1,2],[[3,4],5]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ];
        let magnitudes = vec![
            29,
            21,
            129,
            143,
            1384,
            445,
            791,
            1137,
            3488,
        ];

        for i in 0..inputs.len() {
            let sn = SnailfishNumber::from_string(&String::from(inputs[i]));
            assert_eq!(sn.magnitude(), magnitudes[i]);
        }
    }

    #[test]
    fn test_assignment() {
        let raw_numbers = "
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        ";

        let mut numbers = Vec::new();

        for line in raw_numbers.trim().split("\n") {
            numbers.push(SnailfishNumber::from_string(&String::from(line.trim())));
        }

        let sum: SnailfishNumber = numbers.iter().sum();
        assert_eq!(sum.to_string(), "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        assert_eq!(sum.magnitude(), 4140);
    }
}
