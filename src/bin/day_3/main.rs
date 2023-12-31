fn main() {
    println!("Hello, World");
}
/// Represent all adjacent space to a number
///
/// ****************************
/// *[the length of the number]*
/// ****************************
#[derive(PartialEq, Debug)]
struct Domain {
    origin: usize,
    length: usize,
}
#[derive(PartialEq, Debug)]
enum PuzzlePart {
    Number(usize, u32),
    Symbol(usize),
}

type PuzzleLine = Vec<PuzzlePart>;

impl PuzzlePart {
    fn parse_line(line: &str) -> PuzzleLine {
        let mut result = vec![];
        let mut is_parsing_number = false;
        let mut parsed_number_origin: usize = 0;
        let mut index = 0;
        let mut number_string = String::new();
        if line.is_empty() {
            return result;
        }
        line.chars().for_each(|c| {
            match c {
                '.' => {
                    if is_parsing_number {
                        result.push(PuzzlePart::Number(
                            parsed_number_origin,
                            number_string.parse().unwrap(),
                        ));
                        number_string = String::new();
                        is_parsing_number = false;
                    }
                }
                digit if digit.is_digit(10) => {
                    if !is_parsing_number {
                        parsed_number_origin = index;
                        is_parsing_number = true;
                    };
                    number_string.push(digit);
                }
                _ => result.push(PuzzlePart::Symbol(index)),
            }
            index += 1;
        });
        if is_parsing_number {
            result.push(PuzzlePart::Number(
                parsed_number_origin,
                number_string.parse().unwrap(),
            ));
        }
        result
    }
}
impl Domain {
    fn new(origin: usize, length: usize) -> Self {
        Self { origin, length }
    }

    pub fn include(self: &Self, symbol_origin: usize) -> bool {
        self.origin <= symbol_origin && symbol_origin <= self.origin + self.length
    }
    fn from_number(origin: usize, value: u32) -> Self {
        if origin == 0 {
            Self {
                origin: 0,
                length: value.to_string().len() + 1,
            }
        } else {
            Self {
                origin: origin - 1,
                length: value.to_string().len() + 2,
            }
        }
    }
}

fn filter_adjacent_number(
    previous_line: &PuzzleLine,
    current_line: &PuzzleLine,
    next_line: &PuzzleLine,
) -> Vec<u32> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use crate::{Domain, PuzzleLine, PuzzlePart};

    #[test]
    fn can_convert_number_at_the_begining() {
        let origin = 0;
        let value = 120;
        let expected = Domain::new(0, 4);
        let result = Domain::from_number(origin, value);
        assert_eq!(result, expected)
    }
    #[test]
    fn can_convert_number_in_the_middle() {
        let origin = 10;
        let value = 120;
        let expected = Domain::new(9, 5);
        let result = Domain::from_number(origin, value);
        assert_eq!(result, expected)
    }
    #[test]
    fn can_convert_number_with_length_of_one() {
        let origin = 0;
        let value = 1;
        let expected = Domain::new(0, 2);
        let result = Domain::from_number(origin, value);
        assert_eq!(result, expected)
    }
    #[test]
    fn can_convert_number_with_length_of_many() {
        let origin = 10;
        let value = 1205;
        let expected = Domain::new(9, 6);
        let result = Domain::from_number(origin, value);
        assert_eq!(result, expected)
    }

    #[test]
    fn can_detect_symbol_on_the_right_with_number_at_the_begining() {
        // ****$************
        // 1205$************
        // ****$************
        let symbol = 4;
        let domain = Domain::new(0, 5);
        assert!(domain.include(symbol))
    }
    #[test]
    fn can_detect_symbol_in_between_with_number_at_the_begining() {
        // **$*************
        // 1205************
        // **$*************
        let symbol = 2;
        let domain = Domain::new(0, 5);
        assert!(domain.include(symbol))
    }
    #[test]
    fn can_detect_symbol_on_the_right() {
        // **************$************
        // **********1205$************
        // **************$************
        let symbol = 14;
        let domain = Domain::new(9, 6);
        assert!(domain.include(symbol))
    }
    #[test]
    fn can_detect_symbol_on_the_left() {
        // **********$****************
        // **********$1205************
        // **********$****************
        let symbol = 10;
        let domain = Domain::new(9, 6);
        assert!(domain.include(symbol))
    }
    #[test]
    fn can_detect_symbol_on_the_between() {
        // *************$*************
        // ***********1205************
        // *************$*************
        let symbol = 12;
        let domain = Domain::new(9, 6);
        assert!(domain.include(symbol))
    }
    #[test]
    fn can_not_detect_symbol_outside_of_domain() {
        // *******$*******************
        // *******$***1205************
        // *******$*******************
        let symbol = 7;
        let domain = Domain::new(9, 6);
        assert!(!domain.include(symbol))
    }
    #[test]
    fn can_parse_empty_line() {
        let line = "";
        let expected = PuzzleLine::new();
        let result = PuzzlePart::parse_line(line);
        assert_eq!(result, expected);
    }
    #[test]
    fn can_parse_line_with_number() {
        let line = "..124..";
        let expected = vec![PuzzlePart::Number(2, 124)];
        let result = PuzzlePart::parse_line(line);
        assert_eq!(result, expected);
    }
    #[test]
    fn can_parse_line_with_symbol() {
        let line = "...$..";
        let expected = vec![PuzzlePart::Symbol(3)];
        let result = PuzzlePart::parse_line(line);
        assert_eq!(result, expected);
    }
    #[test]
    fn can_parse_line_with_numbers_symbol() {
        let line = "..25.$..30.~";
        let expected = vec![
            PuzzlePart::Number(2, 25),
            PuzzlePart::Symbol(5),
            PuzzlePart::Number(8, 30),
            PuzzlePart::Symbol(11),
        ];
        let result = PuzzlePart::parse_line(line);
        assert_eq!(result, expected);
    }
}
