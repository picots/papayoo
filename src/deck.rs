use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::card::{Card, Suit};

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /// Builds a full Papayoo deck: 52 standard cards + 8 Jokers
    pub fn new() -> Self {
        let mut cards = Vec::new();

        for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for value in 1..=10 {
                cards.push(Card::new(suit.clone(), value));
            }
        }

        for value in 1..=20 {
            cards.push(Card::new(Suit::Joker, value));
        }

        Deck { cards }
    }

    /// Shuffle the deck
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Deals cards evenly to `num_players` players.
    /// Returns a Vec of hands (one per player).
    pub fn deal(&mut self, num_players: usize) -> Vec<Vec<Card>> {
        self.shuffle();
        let mut hands: Vec<Vec<Card>> = vec![Vec::new(); num_players];
        for (i, card) in self.cards.drain(..).enumerate() {
            hands[i % num_players].push(card);
        }
        hands
    }
}
