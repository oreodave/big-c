use crate::card::{Card, PlayingCard, Rank, Suit};

/*
Because of https://github.com/rust-lang/rust/pull/22299, Range<i64> is not an
ExactSizeIterator => Map<Range<i64>> is not an ESI.  But Range<i32> is an ESI.
*/

impl Rank {
    /// Generate an iterator over all ranks.
    pub fn all() -> [Rank; 13] {
        std::array::from_fn(|i| Rank::try_from(i as i64).unwrap())
    }

    /// Generate an iterator over all ranks after the current one.
    pub fn iter_rest(self) -> impl ExactSizeIterator<Item = Rank> + Clone {
        (((self as i32) + 1)..13).map(|x| Rank::try_from(x as i64).unwrap())
    }

    /// Generate an iterator over all cards within a rank, ordered by Suit.  The
    /// cards are all default initialised w.r.t. deck (0).
    pub fn cards(self) -> [Card; 4] {
        let n = self as usize;
        std::array::from_fn(|x| Card::from((x + (n * 4)) as i64))
    }
}

impl Suit {
    /// Generate an iterator over all suits.
    pub fn iter_all() -> [Suit; 4] {
        std::array::from_fn(|i| Suit::try_from(i as i64).unwrap())
    }

    /// Generate an iterator over all cards within a suit, ordered by Rank.  The
    /// cards are all default initialised w.r.t. deck (0).
    pub fn cards(self) -> [Card; 13] {
        let n = self as usize;
        std::array::from_fn(|x| Card::from((n + (x * 4)) as i64))
    }
}

impl PlayingCard {
    pub fn new(deck: i64, rank: Rank, suit: Suit) -> Self {
        Self { deck, rank, suit }
    }

    /// Return the index of this playing card in [0, 52), ignoring the deck it
    /// belongs to.
    ///
    /// This means any two playing cards from different decks may be equivalent
    /// under PlayingCard::abs if their ranks and suits match, which is used in
    /// the Ordering implementation [[file:ord.rs::impl Ord for PlayingCard {]].
    pub fn abs(&self) -> i64 {
        let rank = self.rank as i64;
        let suit = self.suit as i64;
        (rank * 4) + suit
    }

    /// Generate an iterator over all Playing Cards in the `nth` deck.  By
    /// construction this is in ascending order.
    pub fn iter_all(n: i64) -> [PlayingCard; 52] {
        assert!(n >= 0 && (n * 53) < i64::MAX);
        std::array::from_fn(|x| {
            PlayingCard::try_from((x as i64) + (n * 52)).unwrap()
        })
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

    /// Return the rank of the current Card.  Returns None iff the current Card
    /// is a Joker.
    pub fn rank(&self) -> Option<Rank> {
        self.playing_card().map(|pc| pc.rank)
    }

    /// Return the suit of the current Card.  Returns None iff the current Card
    /// is a Joker.
    pub fn suit(&self) -> Option<Suit> {
        self.playing_card().map(|pc| pc.suit)
    }

    pub fn deck_abs(&self) -> i64 {
        match *self {
            Self::Joker(x) => x,
            Self::PlayingCard(card) => card.abs(),
        }
    }

    /// Generate an iterator over `n` decks of Cards.  By construction, the
    /// iterator is ordered.
    ///
    /// Note that each deck gets two jokers, so there are 2`n` jokers total.
    pub fn iter_all(n: i64) -> impl Iterator<Item = Card> + Clone {
        // We can't know the compile time size of this because we're taking the
        // number of decks at runtime.  So no arrays here.

        // We know the size of this iterator beforehand: it's 54 * n.  Because
        // of https://github.com/rust-lang/rust/pull/22299, Range<i64> is not an
        // ExactSizeIterator => Map<Range<i64>> is not an ESI.  Range<i32> is an
        // ESI, but we can't safely make the range [-2n, 52n] into a Range<i32>
        // because of n being i64.  So we have to leave this as a generic
        // iterator.

        (-(n * 2)..(52 * n)).map(Card::from)
    }
}
