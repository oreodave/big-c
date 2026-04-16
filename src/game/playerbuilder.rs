use crate::game::deck::Deck;

/// Player Builder, which allows the adding of new players.
pub struct PlayerBuilder {
    players: Vec<Deck>,
}

/// A fixed set of players that cannot grow - occurs once players have been
/// picked.
pub type FixedPlayers = Box<[Deck]>;

impl PlayerBuilder {
    /// Construct a new player builder.
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    /// Add a new player with an empty deck to the builder, returning its ID.
    pub fn add(&mut self) -> usize {
        let id = self.players.len();
        self.players.push(Deck::new_empty());
        id
    }

    /// Fix the current number of players for later game stages.
    pub fn into_fixed(self) -> FixedPlayers {
        self.players.into_boxed_slice()
    }
}
