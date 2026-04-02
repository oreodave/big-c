use crate::card::{Card, PlayingCard, Rank, Suit};
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

    // playing_cards is a mapping of all the valid playing cards in the input
    // cardset.
    let mut playing_cards =
        [PlayingCard::new(0, Rank::Three, Suit::Diamond); 5];
    let playing_cards = {
        let cards_slice = &cards[num_jokers..];
        for i in 0..cards_slice.len() {
            playing_cards[i] = match cards_slice[i] {
                Card::Joker(_) => {
                    unreachable!("should be a valid playing card")
                }
                Card::PlayingCard(c) => c,
            };
        }
        &playing_cards[..cards_slice.len()]
    };

    let num_jokers = num_jokers as i32;
    let mut counter_ranks = [0; 13];
    let mut counter_suits = [0; 4];
    for card in playing_cards {
        let rank = card.rank as usize;
        let suit = card.suit as usize;
        counter_ranks[rank] += 1;
        counter_suits[suit] += 1;
    }

    let (highest_rank_freq, num_pairs) = {
        let mut highest_rank_freq = 0;
        let mut num_pairs = 0;
        for i in 0..counter_ranks.len() {
            let rank_freq = counter_ranks[i];
            highest_rank_freq = std::cmp::max(rank_freq, highest_rank_freq);
            if rank_freq == 2 {
                num_pairs += 1
            }
        }
        (highest_rank_freq, num_pairs)
    };
    let is_flush = counter_suits.contains(&playing_cards.len());
    let is_straight = is_straight(num_jokers, playing_cards);

    match (num_pairs, num_jokers, highest_rank_freq) {
        _ if is_straight && is_flush => Some(PokerType::StraightFlush),
        (_, x, y) if x + y == 5 => Some(PokerType::FiveKind),
        (_, x, y) if x + y == 4 => Some(PokerType::FourKind),
        (1, _, 3) | (2, 1, _) => Some(PokerType::FullHouse),
        _ if is_straight => Some(PokerType::Straight),
        _ if is_flush => Some(PokerType::Flush),
        (2, ..) | (1, 1, _) | (_, 2, 1) => Some(PokerType::TwoPair),
        _ => None,
    }
}

/* NOTE: The following functions have a 3rd, even stronger assumption:
   3) No jokers in the sequence of cards provided.
*/

fn is_straight(num_jokers: i32, cards: &[PlayingCard]) -> bool {
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

    let rank_nums = cards.iter().map(|x| x.rank as i32);
    let ord_rank_nums = cards.iter().map(|x| x.rank.ordinary_order());

    strictly_consecutive_numbers(rank_nums, num_jokers)
        || strictly_consecutive_numbers(ord_rank_nums, num_jokers)
}

fn all_same_rank(cards: &[Card]) -> bool {
    let rank = cards[0].rank().unwrap();
    cards[1..].iter().all(|card| rank == card.rank().unwrap())
}

mod traits_display {
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
