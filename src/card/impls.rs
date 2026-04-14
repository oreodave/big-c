use crate::card::{Card, PlayingCard, Rank, Suit};

/*
Because of https://github.com/rust-lang/rust/pull/22299, Range<i64> is not an
ExactSizeIterator => Map<Range<i64>> is not an ESI.  But Range<i32> is an ESI.
*/

impl Rank {
    /// Generate an iterator over all ranks.
    pub fn iter_all() -> [Rank; 13] {
        [
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
            Rank::Two,
        ]
    }

    /// Generate an iterator over all cards within a rank, ordered by Suit.  The
    /// cards are all default initialised w.r.t. deck (0).
    pub fn cards(self) -> [Card; 4] {
        Suit::iter_all().map(move |suit| Card::make_playing_card(0, self, suit))
    }
}

impl Suit {
    /// Generate an iterator over all suits.
    pub fn iter_all() -> [Suit; 4] {
        [Suit::Diamond, Suit::Club, Suit::Heart, Suit::Spade]
    }

    /// Generate an iterator over all cards within a suit, ordered by Rank.  The
    /// cards are all default initialised in terms of deck (0).
    pub fn cards(self) -> [Card; 13] {
        Rank::iter_all().map(move |rank| Card::make_playing_card(0, rank, self))
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

    /// Generate an iterator over all Playing Cards in the `nth` deck.  By
    /// construction this is in ascending order.
    pub fn iter_all(n: i64) -> [PlayingCard; 52] {
        let mut cards: [PlayingCard; 52] = [PlayingCard::default(); 52];
        for i in 0..52 {
            cards[i] = PlayingCard::try_from((i as i64) + (52 * n)).unwrap();
        }
        cards
    }
}

impl Card {
    pub fn make_joker() -> Self {
        Self::Joker(-1)
    }

    pub fn make_playing_card(deck: i64, rank: Rank, suit: Suit) -> Self {
        Self::PlayingCard(PlayingCard::new(deck, rank, suit))
    }

    pub fn is_joker(&self) -> bool {
        matches!(self, Card::Joker(_))
    }

    fn playing_card(&self) -> Option<PlayingCard> {
        match self {
            Card::Joker(_) => None,
            Card::PlayingCard(pc) => Some(*pc),
        }
    }

    pub fn rank(&self) -> Option<Rank> {
        self.playing_card().and_then(|pc| Some(pc.rank))
    }

    pub fn suit(&self) -> Option<Suit> {
        self.playing_card().and_then(|pc| Some(pc.suit))
    }

    pub fn deck_abs(&self) -> i64 {
        match *self {
            Self::Joker(x) => x,
            Self::PlayingCard(card) => card.abs(),
        }
    }

    /// Generate an iterator over `n` decks of Cards.  Each deck is concatenated
    /// together.  By construction, each "deck" of the iterator is in ascending
    /// order.
    ///
    /// Note that each deck gets two jokers.
    pub fn iter_all(n: i64) -> impl Iterator<Item = Card> + Clone {
        // NOTE: I cannot make this into an ExactSizeIterator using the i32
        // trick.  Chain<ESI, ESI> is not an ESI, nor is FlatMap<T,U,T->U>
        // (where T and U are ESIs).
        (-(n * 2)..(52 * n)).map(Card::from)
    }
}
