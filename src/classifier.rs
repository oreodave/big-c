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
    pub fn classify(cards: &[Card]) -> Option<Self> {
        let num_jokers = cards.iter().filter(|c| c.is_joker()).count();
        if cards.len() == 0 || num_jokers == cards.len() {
            None
        } else {
            let mut new_cards: Vec<Card> = Vec::new();
            new_cards.extend_from_slice(cards);
            new_cards.sort();
            match cards.len() {
                1 => Some(Self::Single(new_cards[0])),
                2 => is_pair(num_jokers, new_cards[0], new_cards[1])
                    .then_some(Self::Pair(new_cards[0], new_cards[1])),
                3 => is_triple(
                    num_jokers,
                    new_cards[0],
                    new_cards[1],
                    new_cards[2],
                )
                .then_some(Self::Triple(
                    new_cards[0],
                    new_cards[1],
                    new_cards[2],
                )),
                5 => classify_poker_hand(num_jokers, &new_cards),
                _ => None,
            }
        }
    }
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

/** NOTE: the assumptions of the following functions are:
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
        match_ranks(&[c1, c2])
    }
}

fn is_triple(num_jokers: usize, c1: Card, c2: Card, c3: Card) -> bool {
    if num_jokers == 2 {
        true
    } else if num_jokers == 1 {
        // c2's and c3's rank better match
        match_ranks(&[c2, c3])
    } else {
        // all 3 ranks better match
        match_ranks(&[c1, c2, c3])
    }
}

fn classify_poker_hand(num_jokers: usize, cards: &[Card]) -> Option<Hand> {
    assert!(cards.len() == 5);

    let playing_cards = &cards[num_jokers..];
    let is_straight = consecutive_ranks(playing_cards);
    let is_flush = match_suit(playing_cards);
    let all_same = match_ranks(playing_cards);

    let ptype = match (is_straight, is_flush, all_same, num_jokers) {
        (true, true, _, _) | (_, _, _, 4) => Some(PokerType::StraightFlush),
        (_, _, true, _) => Some(PokerType::FiveKind),
        _ => todo!("Finish all permutations"),
    };

    ptype.and_then(|ptype| Some(Hand::make_poker_hand(ptype, cards)))
}

/*
 NOTE: The following functions have a 3rd, even stronger assumption:
 3) No jokers in the sequence of cards provided.
*/
fn match_ranks(cards: &[Card]) -> bool {
    let rank = cards[0].rank().unwrap();
    cards
        .iter()
        .map(|card| card.rank().unwrap())
        .all(|other_rank| rank == other_rank)
}

fn match_suit(cards: &[Card]) -> bool {
    let suit = cards[0].suit().unwrap();
    cards
        .iter()
        .map(|card| card.suit().unwrap())
        .all(|other_suit| suit == other_suit)
}

fn consecutive_ranks(cards: &[Card]) -> bool {
    for i in 0..(cards.len() - 1) {
        let r1 = cards[i].rank().unwrap() as i32;
        let r2 = cards[i + 1].rank().unwrap() as i32;
        if r1 != r2 + 1 {
            return false;
        }
    }
    return true;
}
