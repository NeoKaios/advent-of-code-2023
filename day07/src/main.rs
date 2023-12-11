use std::{fs, env, collections::HashMap};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Copy, Clone)]
enum Card {
    Joker, C2, C3, C4, C5, C6, C7, C8, C9, T, J, Q, K, A,
}

impl Card {
    fn from_char(c: char, with_joker: bool) -> Card {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => if with_joker {Card::Joker} else {Card::J},
            'T' => Card::T,
            '9' => Card::C9,
            '8' => Card::C8,
            '7' => Card::C7,
            '6' => Card::C6,
            '5' => Card::C5,
            '4' => Card::C4,
            '3' => Card::C3,
            '2' => Card::C2,
            _ => panic!("Unknown card char {c}")
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Type {
    High, OnePair, TwoPair, Three, Full, Four, Five,
}

impl Type {
    fn new(max: i32, second: Option<i32>)->Type {
        match (max,second) {
            (5,_) => Type::Five,
            (4,_) => Type::Four,
            (3,Some(2)) => Type::Full,
            (3,Some(1)) => Type::Three,
            (2,Some(2)) => Type::TwoPair,
            (2,Some(1)) => Type::OnePair,
            (1,_) => Type::High,
            _ => panic!("Weird card amount {max}/{:?}", second)
        }
    }
}

#[derive(Eq,PartialEq, Debug, PartialOrd, Ord)]
struct Hand {
    handtype: Type,
    c1: Card,
    c2: Card,
    c3: Card,
    c4: Card,
    c5: Card,
}

impl Hand {
    fn new(hand: &str, with_joker: bool) -> Hand {
        let cards: Vec<Card> = hand.chars().map(|b| Card::from_char(b, with_joker)).collect();
        let hand = Hand {
            c1: cards[0],
            c2: cards[1],
            c3: cards[2],
            c4: cards[3],
            c5: cards[4],
            handtype: Hand::get_type(&cards, with_joker)
        };
        hand
    }

    fn get_type(cards: &Vec<Card>, with_joker: bool) -> Type {
        let mut dic: HashMap<&Card, i32> = HashMap::new();
        for c in cards {
            if let Some(r) = dic.get_mut(c) {
                *r+=1;
            }
            else {
                dic.insert(c, 1);
            }
        }
        let mut joker_nb = 0;
        if with_joker {
            joker_nb = dic.remove(&Card::Joker).unwrap_or(0);
        }
        let mut vals: Vec<i32> = dic.into_values().collect();
        vals.sort();
        vals.reverse();
        let max = *vals.get(0).unwrap_or(&0) + joker_nb;
        Type::new(max, vals.get(1).map(|i|*i))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content, false);
    compute(&content, true);
}

fn compute(content: &String, with_joker: bool) {
    let mut hands: Vec<(Hand, i32)> = vec!();
    let lines = content.lines();
    for line in lines {
        let mut iter = line.split_whitespace();
        let raw_hand = iter.next().unwrap_or("");
        if raw_hand.len() != 5 { panic!("Got underfull hand: {raw_hand}"); }
        let bid = iter.next().map(|s| s.parse().unwrap_or(0)).unwrap_or(0);
        hands.push((Hand::new(raw_hand, with_joker), bid))
    }
    hands.sort();
    let mut score: i64 = 0;
    for (idx,(_,bid)) in hands.iter().enumerate() {
        score+= (idx as i64 +1)*(*bid as i64);
    }
    println!("{}he score is: {score}", if with_joker {"With jokers, t"} else {"T"});
}
