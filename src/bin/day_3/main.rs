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
enum PuzzlePart {
    Number(usize, u32),
    Symbol(usize),
}

type PuzzleLine = Vec<PuzzlePart>;

impl PuzzlePart {
    fn parse_line(line: &str) -> PuzzleLine {
        unimplemented!()
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
    use crate::Domain;

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
}
