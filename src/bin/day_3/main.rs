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

    fn is_in_Domain(puzzle_part_origin: usize) -> bool {
        unimplemented!()
    }
    fn from_number(origin: usize, value: u32) -> Self {
        let origin = {
            if origin == 0 {
                0
            } else {
                origin - 1
            }
        };
        return Self {
            origin,
            length: value.to_string().len() + 2,
        };
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
        let expected = Domain::new(0, 5);
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
        let expected = Domain::new(0, 3);
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
}
