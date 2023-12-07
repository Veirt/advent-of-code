use std::{cmp::Ordering, collections::HashMap, iter};

#[derive(Debug, Eq, PartialEq, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand_type(card: &str) -> HandType {
    let mut counter = HashMap::new();

    for char in card.chars() {
        *counter.entry(char).or_insert(0 as u32) += 1;
    }

    let values = counter.values().cloned().collect::<Vec<u32>>();

    if values.len() == 1 && values.contains(&5) {
        return HandType::FiveOfAKind;
    } else if values.len() == 2 && values.contains(&4) {
        return HandType::FourOfAKind;
    } else if values.len() == 2 && values.contains(&2) && values.contains(&3) {
        return HandType::FullHouse;
    } else if values.len() == 3 && values.contains(&3) {
        return HandType::ThreeOfAKind;
    } else if values.len() == 3
        && values.contains(&2)
        && values.iter().filter(|v| *v == &2).count() == 2
    {
        return HandType::TwoPair;
    } else if values.len() == 4 && values.contains(&2) {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn get_card_strength(card: char) -> u32 {
    if card.is_ascii_digit() {
        return card.to_digit(10).unwrap();
    } else if card == 'T' {
        return 10;
    } else if card == 'J' {
        return 11;
    } else if card == 'Q' {
        return 12;
    } else if card == 'K' {
        return 13;
    }

    return 14;
}

fn get_rank<'a>(
    grouped_by_type: &mut HashMap<HandType, Vec<(&'a str, u32)>>,
    hand_type: &HandType,
    result: &mut Vec<(&'a str, u32, u32)>,
    rank: &mut u32,
) {
    if let Some(grouped_by_type) = grouped_by_type.get_mut(hand_type) {
        grouped_by_type.sort_by(|hand1, hand2| compare_hand_strength(&hand1.0, &hand2.0));
    }

    let hand_type_items = grouped_by_type.get(hand_type);
    if let Some(hand_type_items) = hand_type_items {
        for (hand, bid) in hand_type_items {
            result.push((hand, *rank, *bid));
            *rank += 1;
        }
    }
}

fn compare_hand_strength(hands1: &str, hands2: &str) -> Ordering {
    for (card_hands_1, card_hands_2) in iter::zip(hands1.chars(), hands2.chars()) {
        let strength1 = get_card_strength(card_hands_1);
        let strength2 = get_card_strength(card_hands_2);

        if strength1 == strength2 {
            continue;
        } else if strength1 > strength2 {
            return Ordering::Greater;
        } else if strength1 < strength2 {
            return Ordering::Less;
        }
    }

    unreachable!();
}

fn process(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut grouped_by_type: HashMap<HandType, Vec<(&str, u32)>> = HashMap::new();

    for line in lines {
        let splitted = line.split(' ').collect::<Vec<&str>>();
        let card = splitted[0];
        let bid = splitted[1]
            .parse::<u32>()
            .expect("should be parsed correctly");

        let hand_type = get_hand_type(card);
        grouped_by_type
            .entry(hand_type)
            .or_default()
            .push((card, bid));
    }

    // hand, rank, bid
    let mut result: Vec<(&str, u32, u32)> = vec![];

    let mut rank = 1;

    get_rank(
        &mut grouped_by_type,
        &HandType::HighCard,
        &mut result,
        &mut rank,
    );
    get_rank(
        &mut grouped_by_type,
        &HandType::OnePair,
        &mut result,
        &mut rank,
    );
    get_rank(
        &mut grouped_by_type,
        &HandType::TwoPair,
        &mut result,
        &mut rank,
    );
    get_rank(
        &mut grouped_by_type,
        &HandType::ThreeOfAKind,
        &mut result,
        &mut rank,
    );
    get_rank(
        &mut grouped_by_type,
        &HandType::FullHouse,
        &mut result,
        &mut rank,
    );
    get_rank(
        &mut grouped_by_type,
        &HandType::FourOfAKind,
        &mut result,
        &mut rank,
    );
    get_rank(
        &mut grouped_by_type,
        &HandType::FiveOfAKind,
        &mut result,
        &mut rank,
    );

    let mut ans: u64 = 0;
    for (_, rank, bid) in &result {
        ans += (*bid as u64) * (*rank as u64);
    }

    ans
}

fn main() {
    let input = include_str!("../../../day07.input.txt");

    let result = process(input);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hand_type() {
        assert!(matches!(get_hand_type("AAAAA"), HandType::FiveOfAKind));
        assert!(matches!(get_hand_type("AA8AA"), HandType::FourOfAKind));
        assert!(matches!(get_hand_type("23332"), HandType::FullHouse));
        assert!(matches!(get_hand_type("TTT98"), HandType::ThreeOfAKind));
        assert!(matches!(get_hand_type("23432"), HandType::TwoPair));
        assert!(matches!(get_hand_type("A23A4"), HandType::OnePair));
        assert!(matches!(get_hand_type("23456"), HandType::HighCard));
        assert!(matches!(get_hand_type("32T3K"), HandType::OnePair));
        assert!(matches!(get_hand_type("KK677"), HandType::TwoPair));
        assert!(matches!(get_hand_type("KTJJT"), HandType::TwoPair));
        assert!(matches!(get_hand_type("T55J5"), HandType::ThreeOfAKind));
        assert!(matches!(get_hand_type("QQQJA"), HandType::ThreeOfAKind));
    }

    #[test]
    fn test_compare_card_strength() {
        assert_eq!(compare_hand_strength("KK677", "KTJJT"), Ordering::Greater);
        assert_eq!(compare_hand_strength("KTJJT", "KK677"), Ordering::Less);

        assert_eq!(compare_hand_strength("QQQJA", "T55J5"), Ordering::Greater);
        assert_eq!(compare_hand_strength("T55J5", "QQQJA"), Ordering::Less);
    }

    #[test]
    fn sample() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        assert_eq!(process(input), 6440);
    }

    #[test]
    fn bigboy() {
        let input = include_str!("../../../day07.bigboy.txt");

        assert_eq!(process(input), 6678229757944529u64);
    }
}
