use crate::Card;

#[derive(Debug)]
pub enum PokerType {
    TwoPair,
    Flush,
    Straight,
    FullHouse,
    FourKind,
    FiveKind,
    StraightFlush,
}

#[derive(Debug)]
pub enum Hand {
    Single(Card),
    Pair(Card, Card),
    Triple(Card, Card, Card),
    Poker {
        poker_type: PokerType,
        c1: Card,
        c2: Card,
        c3: Card,
        c4: Card,
        c5: Card,
    },
}

impl Hand {
    // Stupid shorthand method of generating poker hands
    fn make_poker_hand(poker_type: PokerType, cards: &[Card]) -> Self {
        Self::Poker {
            poker_type,
            c1: cards[0],
            c2: cards[1],
            c3: cards[2],
            c4: cards[3],
            c5: cards[4],
        }
    }
}

pub fn classify(cards: &[Card]) -> Option<Hand> {
    let num_jokers = cards.iter().filter(|c| c.is_joker()).count();
    if cards.is_empty() || num_jokers == cards.len() {
        return None;
    }

    let mut new_cards: Vec<Card> = Vec::new();
    new_cards.extend_from_slice(cards);
    new_cards.sort();
    match new_cards.len() {
        1 => Some(Hand::Single(new_cards[0])),
        2 => is_pair(num_jokers, new_cards[0], new_cards[1])
            .then_some(Hand::Pair(new_cards[0], new_cards[1])),
        3 => is_triple(num_jokers, new_cards[0], new_cards[1], new_cards[2])
            .then_some(Hand::Triple(new_cards[0], new_cards[1], new_cards[2])),
        5 => classify_poker_hand(num_jokers, &new_cards)
            .map(|ptype| Hand::make_poker_hand(ptype, cards)),
        _ => None,
    }
}

/* NOTE: the assumptions of the following functions are:
   1) The arguments are not all jokers
   2) The arguments are sorted i.e. c_n < c_n+1 for all n.

   Consequences:
   - Any jokers are on the lower end of the sequence of cards, due to (2)
   - If l is the number of cards and (c_n)_0^(l-1) are all jokers, hand may be
     classified as the strongest type possible.
       - because (2), all jokers are sorted to the bottom.
*/

fn is_pair(num_jokers: usize, c1: Card, c2: Card) -> bool {
    if num_jokers == 1 {
        true
    } else {
        // Otherwise, their ranks better match
        all_same_rank(&[c1, c2])
    }
}

fn is_triple(num_jokers: usize, c1: Card, c2: Card, c3: Card) -> bool {
    if num_jokers == 2 {
        true
    } else if num_jokers == 1 {
        // c2's and c3's rank better match
        all_same_rank(&[c2, c3])
    } else {
        // all 3 ranks better match
        all_same_rank(&[c1, c2, c3])
    }
}

fn classify_poker_hand(num_jokers: usize, cards: &[Card]) -> Option<PokerType> {
    // NOTE: |cards| = 5
    // NOTE: num_jokers in [0, 4]

    // FIXME: So ugly.  Any way we can make this better?

    let num_jokers = num_jokers as i32;
    let playing_cards = &cards[num_jokers..];

    let (counter_ranks, counter_suits) = count_cards(playing_cards);
    let highest_suit_freq = *counter_suits.iter().max().unwrap();
    let highest_rank_freq = *counter_ranks.iter().max().unwrap();
    let num_pairs = counter_ranks.iter().filter(|&&x| x == 2).count();

    let is_straight = is_straight(num_jokers, playing_cards);
    let is_flush = highest_suit_freq == playing_cards.len() as i32;
    if is_straight && is_flush || num_jokers == 4 {
        Some(PokerType::StraightFlush)
    } else if num_jokers + highest_rank_freq == 5 {
        Some(PokerType::FiveKind)
    } else if num_jokers + highest_rank_freq == 4 {
        Some(PokerType::FourKind)
    } else if (num_jokers == 1 && num_pairs == 2)
        || (num_pairs > 0 && highest_rank_freq == 3)
    {
        Some(PokerType::FullHouse)
    } else if is_straight {
        Some(PokerType::Straight)
    } else if is_flush && highest_rank_freq == 1 {
        Some(PokerType::Flush)
    } else if (num_pairs == 2) || (num_jokers == 2 && highest_rank_freq == 1) {
        Some(PokerType::TwoPair)
    } else {
        None
    }
}

/*
 NOTE: The following functions have a 3rd, even stronger assumption:
 3) No jokers in the sequence of cards provided.
*/

fn count_cards(cards: &[Card]) -> ([i32; 13], [i32; 4]) {
    let mut counter_rank = [0; 13];
    let mut counter_suit = [0; 4];
    cards
        .iter()
        .map(|card| (card.rank().unwrap(), card.suit().unwrap()))
        .for_each(|(rank, suit)| {
            counter_rank[rank as usize] += 1;
            counter_suit[suit as usize] += 1;
        });
    (counter_rank, counter_suit)
}

fn all_same_rank(cards: &[Card]) -> bool {
    let rank = cards[0].rank().unwrap();
    cards
        .iter()
        .map(|card| card.rank().unwrap())
        .all(|other_rank| rank == other_rank)
}

fn is_straight(num_jokers: i32, cards: &[Card]) -> bool {
    /** Given a slice `nums` (presumed ascending ordered) and the amount of allowed
     * `gaps`, figure out if the nums are actually a consecutive sequence.
     */
    fn strictly_consecutive_numbers<I>(mut nums: I, mut gaps: i32) -> bool
    where
        I: Iterator<Item = i32>,
    {
        let mut prev = match nums.next() {
            Some(n) => n,
            None => unreachable!("Iterator should not be empty"),
        };
        for m in nums {
            let diff = m - prev;
            if diff == 0 || diff - 1 > gaps {
                return false;
            }
            gaps -= diff - 1;
            if gaps < 0 {
                return false;
            }
            prev = m;
        }
        true
    }

    let ranks = cards.iter().map(|x| x.rank().unwrap() as i32);

    if !strictly_consecutive_numbers(ranks, num_jokers) {
        // If we don't have a strictly consecutive sequence, try using an
        // ordinary order where Ace is the lowest rank and king is the highest.
        let ranks = cards.iter().map(|x| x.rank().unwrap().ordinary_order());
        strictly_consecutive_numbers(ranks, num_jokers)
    } else {
        true
    }
}

mod traits {
    use super::*;
    use std::fmt::{Display, Formatter, Result};

    impl Display for Hand {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Hand::Single(c1) => write!(f, "Single[{}]", c1),
                Hand::Pair(c1, c2) => write!(f, "Pair[{}, {}]", c1, c2),
                Hand::Triple(c1, c2, c3) => {
                    write!(f, "Triple[{}, {}, {}]", c1, c2, c3)
                }
                Hand::Poker {
                    poker_type,
                    c1,
                    c2,
                    c3,
                    c4,
                    c5,
                } => {
                    write!(
                        f,
                        "Poker[{:?}: {}, {}, {}, {}, {}]",
                        poker_type, c1, c2, c3, c4, c5
                    )
                }
            }
        }
    }
}
