use std::fs;

fn main() {
    let games = fs::read_to_string("data/conundrum.txt").unwrap();
    let bag = Bag::new(14, 13, 12);
    let possible_games = games_possible_for_bag(games.as_str(), bag);
    let connudrum: u32 = possible_games.iter().sum();
    let power = games_power(games.as_str());
    println!("Conundrum {:?} to the power of {}", connudrum, power);
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
    reveal.split(',').for_each(|cube_count| {
        let split: Vec<&str> = cube_count.trim_start().split(' ').collect();
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
    game.split(';').for_each(|reveal| {
        let reveal_bag = bag_from_reveal(reveal);
        bag.blue = std::cmp::max(bag.blue, reveal_bag.blue);
        bag.red = std::cmp::max(bag.red, reveal_bag.red);
        bag.green = std::cmp::max(bag.green, reveal_bag.green);
    });
    bag
}

fn games_power(games: &str) -> u32 {
    if games.is_empty() {
        return 0;
    }
    games
        .split('\n')
        .map(parse_game)
        .map(|(_games, bag)| bag.green * bag.red * bag.blue)
        .sum()
}
fn games_possible_for_bag(games: &str, bag: Bag) -> Vec<u32> {
    if games.is_empty() {
        return vec![];
    }
    games
        .split('\n')
        .map(parse_game)
        .filter_map(|(game_number, minimum_bag)| {
            if minimum_bag.blue <= bag.blue
                && minimum_bag.green <= bag.green
                && minimum_bag.red <= bag.red
            {
                return Some(game_number);
            }
            None
        })
        .collect()
}

fn parse_game(game: &str) -> (u32, Bag) {
    if game.len() < 5 {
        panic!("Could not parse games")
    }
    let split: Vec<&str> = game[5..].split(':').collect();
    if split.len() == 2 {
        let game_number: u32 = split[0].parse().unwrap();
        let bag = minimum_bag_for_game(split[1]);
        return (game_number, bag);
    }
    panic!("Could not parse games")
}

mod test {
    #![allow(unused)]
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

    fn assert_games_possible_for_bag(games: &str, bag: Bag, expected: Vec<u32>) {
        let result = games_possible_for_bag(games, bag);
        assert_eq!(result, expected)
    }

    #[test]
    fn can_find_possible_games_for_empty_set() {
        let games = "";
        let bag = Bag::empty();
        let expected = vec![];
        assert_games_possible_for_bag(games, bag, expected);
    }

    #[test]
    fn can_find_possible_games_for_non_empty_set() {
        let games = "Game 1: 3 green, 7 blue; 7 red, 4 blue; 5 blue, 6 red, 2 green
Game 2: 9 green; 8 green, 4 blue; 6 blue, 2 red, 1 green; 4 green, 1 blue; 5 blue, 2 green, 2 red
Game 3: 3 red, 1 green, 5 blue; 1 red; 3 blue, 4 red; 3 blue, 1 green, 5 red";
        let bag = Bag::new(8, 4, 8);
        let expected = vec![1, 3];
        assert_games_possible_for_bag(games, bag, expected);
    }
}
