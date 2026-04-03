use crate::card::Card;
use crate::modes::pair::Pair;
use crate::modes::single::Single;

struct Player {
    name: String,
    hand: Vec<Card>,
}

pub enum RoundHand {
    Single(Single),
    Pair(Pair),
}

pub enum RoundType {
    Singles,
    Pairs,
}

pub struct Round {
    pub round_type: RoundType,
    pub last_hand: Option<RoundHand>,
    pub ticks: usize,
    pub ticks_last_hand: usize,
}

/** Fat structure representing a Game - this is passed through the stages of a
Game and initialised/utilised. */
struct Game {
    players: Vec<Player>,
    ordering: Vec<usize>,

    played_deck: Vec<Card>,
    free_deck: Vec<Card>,

    round: Round,
}

/** Struct representing a Game in the Waiting stage.  Essentially an unfinished
collection of players. */
struct Waiting(Game);

/** Struct representing a Game undergoing election following the Waiting Stage.
There should be a fixed number of players now and they should have a hand.

Election determines the initial playing order.  It works as follows:
1) Each player presents 1 Playing Card.
2) The ordering of each card represents the order of play for the respective
   player - lowest goes first.
3) If there are any conflicts due to equivalence of card, randomly pick ordering
   between those affected.

*/
struct Election(Waiting);

/** Struct representing a Game deciding the round type.  Occurs either after
Election or after End of Round.

players[0] decides the round type by playing a hand of that type, kicking off
the game.

*/
struct RoundDecision(Game);

/** Struct representing an in-progress round.  Occurs after a Round Decision.

During an in-progress game we follow the player ordering, where each player
either:
1) Plays a valid Hand of that Round Type which is better than the previously
   played Hand.
2) Skips their turn, picking up a card from the unplayed deck.

If (1) cannot be achieved with the current cards, players are forced to skip.

Round ending conditions:
- If there are n players, and n-1 players skip, then the round is won by the
  player who played the last hand.  The game returns to a RoundDecision.
- If a player plays all their cards, then the game is won by that player.  The
  game stops.

*/
struct InProgress(RoundDecision);
