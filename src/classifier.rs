use crate::card::Card;
use crate::hand::{Hand, PokerType};

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

    let mut cards_cpy = [Card::from(0); 5];
    cards_cpy.copy_from_slice(cards);
    cards_cpy.sort();
    match cards_cpy.len() {
        1 => Some(Hand::Single(cards_cpy[0])),
        2 => is_pair(num_jokers, cards_cpy[0], cards_cpy[1])
            .then_some(Hand::Pair(cards_cpy[0], cards_cpy[1])),
        3 => is_triple(num_jokers, cards_cpy[0], cards_cpy[1], cards_cpy[2])
            .then_some(Hand::Triple(cards_cpy[0], cards_cpy[1], cards_cpy[2])),
        5 => try_poker_hand(num_jokers, &cards_cpy)
            .map(|ptype| Hand::make_poker_hand(ptype, &cards_cpy)),
        _ => None,
    }
}

/* NOTE: the assumptions of the following functions are:
   1) The cards are not all jokers
   2) The cards are sorted i.e. c_n < c_n+1 for all n.

  We can make these assumptions because of how `classify` calls these functions.

   Consequences:
   - Any jokers are on the lower end of the sequence of cards, due to (2)
   - If l is the number of cards and there are l-1 jokers, hand may be
     classified as the strongest type possible for that hand type.
*/

fn is_pair(num_jokers: usize, c1: Card, c2: Card) -> bool {
    (num_jokers == 1) || all_same_rank(&[c1, c2])
}

fn is_triple(num_jokers: usize, c1: Card, c2: Card, c3: Card) -> bool {
    match num_jokers {
        2 => true,
        1 => all_same_rank(&[c2, c3]),
        _ => all_same_rank(&[c1, c2, c3]),
    }
}

fn try_poker_hand(num_jokers: usize, cards: &[Card]) -> Option<PokerType> {
    // FIXME: So ugly.  Any way we can make this better?

    let playing_cards = &cards[num_jokers..];
    let num_jokers = num_jokers as i32;

    let mut counter_ranks = [0; 13];
    let mut counter_suits = [0; 13];
    for card in playing_cards {
        let rank = card.rank().unwrap() as usize;
        let suit = card.suit().unwrap() as usize;
        counter_ranks[rank] += 1;
        counter_suits[suit] += 1;
    }

    let highest_rank_freq = *counter_ranks.iter().max().unwrap();
    let num_pairs = counter_ranks.iter().filter(|&count| *count == 2).count();
    let is_straight = is_straight(num_jokers, playing_cards);
    let is_flush = counter_suits.contains(&playing_cards.len());

    if is_straight && is_flush || num_jokers == 4 {
        Some(PokerType::StraightFlush)
    } else if num_jokers + highest_rank_freq == 5 {
        Some(PokerType::FiveKind)
    } else if num_jokers + highest_rank_freq == 4 {
        Some(PokerType::FourKind)
    } else if (num_pairs == 1 && highest_rank_freq == 3)
        || (num_jokers == 1 && num_pairs == 2)
    {
        Some(PokerType::FullHouse)
    } else if is_straight {
        Some(PokerType::Straight)
    } else if is_flush {
        Some(PokerType::Flush)
    } else if (num_pairs == 2)
        || (num_jokers == 1 && num_pairs == 1)
        || (num_jokers == 2 && highest_rank_freq == 1)
    {
        Some(PokerType::TwoPair)
    } else {
        None
    }
}

/* NOTE: The following functions have a 3rd, even stronger assumption:
   3) No jokers in the sequence of cards provided.
*/

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
            if diff <= 0 || diff - 1 > gaps {
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

    let rank_nums = cards.iter().map(|x| x.rank().unwrap() as i32);
    let ord_rank_nums =
        cards.iter().map(|x| x.rank().unwrap().ordinary_order());

    strictly_consecutive_numbers(rank_nums, num_jokers)
        || strictly_consecutive_numbers(ord_rank_nums, num_jokers)
}

fn all_same_rank(cards: &[Card]) -> bool {
    let rank = cards[0].rank().unwrap();
    cards[1..]
        .iter()
        .map(|card| card.rank().unwrap())
        .all(|other_rank| rank == other_rank)
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
