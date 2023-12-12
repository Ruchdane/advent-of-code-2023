fn main() {
    println!("Hello, World");
}
/// Represent all adjacent space to a number
///
/// ****************************
/// *[the length of the number]*
/// ****************************
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
    fn is_in_Domain(puzzle_part_origin: usize) -> bool {
        unimplemented!()
    }
    fn domain(part: PuzzlePart) -> Self {
        unimplemented!()
    }
}

fn filter_adjacent_number(
    previous_line: &PuzzleLine,
    current_line: &PuzzleLine,
    next_line: &PuzzleLine,
) -> Vec<u32> {
    unimplemented!()
}
