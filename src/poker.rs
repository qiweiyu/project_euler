use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Suit { H, C, S, D }

#[derive(Debug, Clone)]
pub struct Card {
    suit: Suit,
    val: i32,
}

impl Card {
    pub fn new(val: char, suit: char) -> Self {
        Card {
            suit: match suit {
                'H' => Suit::H,
                'C' => Suit::C,
                'S' => Suit::S,
                _ => Suit::D,
            },
            val: match val {
                n @ '2'..='9' => n as i32 - '0' as i32,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Poker {
    HighCard(Vec<i32>),
    OnePair(Vec<i32>),
    TwoPairs(Vec<i32>),
    TreeKind(Vec<i32>),
    Straight(i32),
    Flush(Vec<i32>),
    FullHouse(Vec<i32>),
    Four(Vec<i32>),
    StraightFlush(i32),
}

impl Poker {
    pub fn new(mut list: Vec<Card>) -> Self {
        list.sort_by(|a, b| { b.val.cmp(&a.val) });
        let is_straight = Self::_check_straight(&list);
        let is_flush = Self::_check_flush(&list);
        match (is_straight, is_flush) {
            (true, true) => Poker::StraightFlush(list.get(0).unwrap().val),
            (true, false) => Poker::Straight(list.get(0).unwrap().val),
            (false, true) => {
                Poker::Flush(Self::_fetch_val_to_list(&list))
            }
            _ => {
                let count_val = Self::_count_val(&list);
                if let Some(l4) = count_val.get(&4) {
                    Poker::Four(vec![l4[0], count_val.get(&1).unwrap()[0]])
                } else if let Some(l3) = count_val.get(&3) {
                    if let Some(l2) = count_val.get(&2) {
                        Poker::FullHouse(vec![l3[0], l2[0]])
                    } else {
                        let l1 = count_val.get(&1).unwrap();
                        Poker::TreeKind(vec![l3[0], l1[0], l1[1]])
                    }
                } else if let Some(l2) = count_val.get(&2) {
                    let l1 = count_val.get(&1).unwrap();
                    if l2.len() == 2 {
                        Poker::TwoPairs(vec![l2[0], l2[1], l1[0]])
                    } else {
                        Poker::OnePair(vec![l2[0], l1[0], l1[1], l1[2]])
                    }
                } else {
                    let l1 = count_val.get(&1).unwrap();
                    Poker::HighCard(l1.clone())
                }
            }
        }
    }

    fn _fetch_val_to_list(list: &Vec<Card>) -> Vec<i32> {
        let mut res = vec![];
        for card in list {
            res.push(card.val);
        }
        res
    }

    fn _check_straight(list: &Vec<Card>) -> bool {
        let mut is_straight = true;
        let mut current_val = list.get(0).unwrap().val + 1;
        for card in list {
            if card.val != current_val - 1 {
                is_straight = false;
                break;
            } else {
                current_val -= 1;
            }
        }
        is_straight
    }

    fn _check_flush(list: &Vec<Card>) -> bool {
        let mut is_flush = true;
        let first_suit = &list.get(0).unwrap().suit;
        for card in list {
            if &
                card.suit != first_suit {
                is_flush = false;
                break;
            }
        }
        is_flush
    }

    fn _count_val(list: &Vec<Card>) -> HashMap<i32, Vec<i32>> {
        let val_list = Self::_fetch_val_to_list(list);
        let mut count_map = HashMap::new();
        for val in val_list {
            count_map.entry(val).and_modify(|x| { *x = *x + 1 }).or_insert(1);
        }
        let mut res = HashMap::new();
        for (val, count) in count_map {
            res.entry(count).and_modify(|x: &mut Vec<i32>| {
                x.push(val);
                x.sort();
                x.reverse();
            }).or_insert(vec![val]);
        }
        res
    }

    fn _val_to_num(list: &Vec<i32>) -> i32 {
        let mut res = 0;
        for i in list {
            res = res * 15;
            res = res + i;
        }
        res
    }

    fn _type_to_tuple(&self) -> (i32, Vec<i32>) {
        match self {
            Poker::HighCard(v) => (1, v.clone()),
            Poker::OnePair(v) => (2, v.clone()),
            Poker::TwoPairs(v) => (3, v.clone()),
            Poker::TreeKind(v) => (4, v.clone()),
            Poker::Straight(v) => (5, vec![*v]),
            Poker::Flush(v) => (6, v.clone()),
            Poker::FullHouse(v) => (7, v.clone()),
            Poker::Four(v) => (8, v.clone()),
            Poker::StraightFlush(v) => (9, vec![*v])
        }
    }
}

use std::cmp::Ordering;

impl PartialOrd for Poker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (self_level, self_val) = self._type_to_tuple();
        let (other_level, other_val) = other._type_to_tuple();
        if self_level == other_level {
            Some(Self::_val_to_num(&self_val).cmp(&Self::_val_to_num(&other_val)))
        } else {
            Some(self_level.cmp(&other_level))
        }
    }
}

impl Ord for Poker {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}