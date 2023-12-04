fn main() {
    println!("Hello, World");
}
#[derive(Debug, PartialEq)]
struct Bag {
    blue: u32,
    green: u32,
    red: u32,
}

impl Bag {
    fn new(blue: u32, green: u32, red: u32) -> Self {
        Self { blue, green, red }
    }
    fn empty() -> Self {
        Self {
            blue: 0,
            green: 0,
            red: 0,
        }
    }
}
fn bag_from_reveal(reveal: &str) -> Bag {
    let mut bag = Bag::empty();
    if reveal.is_empty() {
        return bag;
    }
    reveal.split(',').into_iter().for_each(|cube_count| {
        let split: Vec<&str> = cube_count.trim_start().split(' ').collect();
        println!("{:?}", split);
        let count: u32 = split[0].parse().unwrap();
        let color = split[1];
        match color {
            "green" => bag.green = count,
            "red" => bag.red = count,
            "blue" => bag.blue = count,
            _ => panic!("Unknown color"),
        };
    });
    bag
}
fn minimum_bag_for_game(game: &str) -> Bag {
    let mut bag = Bag::empty();
    if game.is_empty() {
        return bag;
    }
    game.split(';').into_iter().for_each(|reveal| {
        let reveal_bag = bag_from_reveal(reveal);
        bag.blue = std::cmp::max(bag.blue, reveal_bag.blue);
        bag.red = std::cmp::max(bag.red, reveal_bag.red);
        bag.green = std::cmp::max(bag.green, reveal_bag.green);
    });
    bag
}

fn games_possible_for_bag(games: &str, bag: Bag) {
    todo!()
}

mod test {
    use crate::*;
    fn assert_reveal_conversion(reveal: &str, expected: Bag) {
        let result = bag_from_reveal(reveal);
        assert_eq!(expected, result);
    }

    #[test]
    fn can_convert_empty_reveal() {
        assert_reveal_conversion("", Bag::empty())
    }
    #[test]
    fn can_convert_single_green_reveal() {
        assert_reveal_conversion("2 green", Bag::new(0, 2, 0))
    }
    #[test]
    fn can_convert_single_red_reveal() {
        assert_reveal_conversion("14 red", Bag::new(0, 0, 14))
    }
    #[test]
    fn can_convert_single_blue_reveal() {
        assert_reveal_conversion("1 blue", Bag::new(1, 0, 0));
    }
    #[test]
    fn can_convert_two_reveal() {
        assert_reveal_conversion("3 blue, 4 red", Bag::new(3, 0, 4));
    }
    #[test]
    fn can_convert_complete_reveal() {
        assert_reveal_conversion("1 green, 3 red, 6 blue", Bag::new(6, 1, 3));
    }

    fn assert_minimum_bag_for_game(game: &str, expected: Bag) {
        let result = minimum_bag_for_game(game);
        assert_eq!(expected, result);
    }

    #[test]
    fn can_find_minimum_for_game_with_no_reveal() {
        assert_minimum_bag_for_game("", Bag::empty());
    }
    #[test]
    fn can_find_minimum_for_game_with_one_reveal() {
        assert_minimum_bag_for_game("1 green, 3 red, 6 blue", Bag::new(6, 1, 3));
    }
    #[test]
    fn can_find_minimum_for_game_with_multiple_reveal() {
        let game = "1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let expected = Bag::new(15, 3, 14);
        assert_minimum_bag_for_game(game, expected);
    }
}
