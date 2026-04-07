use crate::card::{Card, PlayingCard, Rank, Suit};
use std::fmt::{Display, Formatter};

/*
In C this would look something like:
switch (self) {
  case RANK_JACK: fprintf(fp, "J"); break;
  case RANK_QUEEN: fprintf(fp, "Q"); break;
  case RANK_KING: fprintf(fp, "K"); break;
  case RANK_ACE: fprintf(fp, "A"); break;
  case RANK_TWO: fprintf(fp, "2"); break;
  default: fprintf(fp, "%d", self + 3); break;
}

but I forgive Rust (for now 👀).
 */

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
            }
        )
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Diamond => "♦",
                Suit::Club => "♣",
                Suit::Heart => "♥",
                Suit::Spade => "♠",
            }
        )
    }
}

impl Display for PlayingCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.rank, self.suit)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Joker(_) => write!(f, "Joker"),
            Card::PlayingCard(card) => write!(f, "{}", card),
        }
    }
}
