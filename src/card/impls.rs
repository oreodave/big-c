use crate::card::{Card, PlayingCard, Rank, Suit};

impl Rank {
    pub fn ordinary_order(&self) -> i32 {
        ((*self as i32) + 2) % 13
    }

    /** Generate an iterator over all ranks. */
    pub fn iter_all() -> impl ExactSizeIterator<Item = Rank> {
        (0i32..13).map(|n| Rank::try_from(n as i64).unwrap())
    }

    /** Generate an iterator over all cards within a rank, ordered by Suit. */
    pub fn cards(self) -> impl Iterator<Item = Card> {
        let n = self as i64;
        ((n * 4)..((n + 1) * 4)).map(Card::from)
    }
}

impl Suit {
    /** Generate an iterator over all suits. */
    pub fn iter_all() -> impl Iterator<Item = Suit> {
        (0..4).filter_map(|x| Suit::try_from(x).ok())
    }

    /** Generate an iterator over all cards within a suit, ordered by Suit. */
    pub fn cards(self) -> impl Iterator<Item = Card> {
        Rank::iter_all().map(move |rank| Card::make_playing_card(rank, self))
    }
}

impl PlayingCard {
    pub fn new(deck: i64, rank: Rank, suit: Suit) -> Self {
        Self { deck, rank, suit }
    }

    pub fn abs(&self) -> i64 {
        let rank = self.rank as i64;
        let suit = self.suit as i64;
        (rank * 4) + suit
    }

    /** Generate an iterator over all Playing Cards in a fixed deck.  By
    construction this is in ascending order. */
    pub fn iter_all(deck: usize) -> impl Iterator<Item = Self> {
        let deck = deck as i64;
        ((deck * 52)..((deck + 1) * 52))
            .filter_map(|x| PlayingCard::try_from(x).ok())
    }

    /** Return the Playing Card after the current one in terms of ordering.

    Returns None if self is 2 of Spades (the highest possible Playing Card).
    Respects deck of self. */
    pub fn next(&self) -> Option<PlayingCard> {
        match *self {
            PlayingCard {
                rank: Rank::Two,
                suit: Suit::Spade,
                ..
            } => None,
            card => PlayingCard::try_from(i64::from(card) + 1).ok(),
        }
    }

    /** Return the Playing Card before the current one in terms of ordering.

    Returns None if self is 3 of Diamonds (the lowest possible Playing Card).
    Respects deck of self. */
    pub fn prev(&self) -> Option<PlayingCard> {
        match *self {
            PlayingCard {
                rank: Rank::Three,
                suit: Suit::Diamond,
                ..
            } => None,
            card => PlayingCard::try_from(i64::from(card) - 1).ok(),
        }
    }
}

impl Card {
    pub fn make_joker() -> Self {
        Self::Joker(-1)
    }

    pub fn make_playing_card(rank: Rank, suit: Suit) -> Self {
        Self::PlayingCard(PlayingCard::new(0, rank, suit))
    }

    pub fn deck_abs(&self) -> i64 {
        match *self {
            Self::Joker(x) => x,
            Self::PlayingCard(card) => card.abs(),
        }
    }

    /** Generate an iterator over a `n` decks of Cards.  Each deck is
    concatenated together.  By construction, each "deck" of the iterator is in
    ascending order.

    Note that each deck gets two jokers.
     */
    pub fn iter_all(n: usize) -> impl Iterator<Item = Card> {
        (-((n as i64) * 2)..0).map(Card::from).chain(
            (0..n)
                .flat_map(PlayingCard::iter_all)
                .map(Card::PlayingCard),
        )
    }

    /** Return the Card after the current one in terms of ordering.

    Returns None if self is a joker, or based on PlayingCard::next.*/
    pub fn next(&self) -> Option<Card> {
        match *self {
            Card::Joker(_) => None,
            Card::PlayingCard(card) => card.next().map(Card::PlayingCard),
        }
    }

    /** Return the Card before the current one in terms of ordering.

    Returns None if self is a joker, or based on PlayingCard::prev.*/
    pub fn prev(&self) -> Option<Card> {
        match *self {
            Card::Joker(_) => None,
            Card::PlayingCard(card) => card.prev().map(Card::PlayingCard),
        }
    }

    pub fn is_joker(&self) -> bool {
        matches!(self, Self::Joker(_))
    }

    pub fn playing_card(&self) -> Option<PlayingCard> {
        match *self {
            Self::Joker(_) => None,
            Self::PlayingCard(card) => Some(card),
        }
    }

    pub fn rank(&self) -> Option<Rank> {
        self.playing_card().map(|pc| pc.rank)
    }

    pub fn suit(&self) -> Option<Suit> {
        self.playing_card().map(|pc| pc.suit)
    }
}
