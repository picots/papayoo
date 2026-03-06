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
            for value in 1..=13 {
                cards.push(Card::new(suit.clone(), value));
            }
        }

        for value in 1..=8 {
            cards.push(Card::new(Suit::Joker, value));
        }

        Deck { cards }
    }

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
        // Sort each hand for nicer display
        for hand in &mut hands {
            hand.sort_by(|a, b| {
                let sa = suit_order(&a.suit);
                let sb = suit_order(&b.suit);
                sa.cmp(&sb).then(a.value.cmp(&b.value))
            });
        }
        hands
    }
}

fn suit_order(suit: &crate::card::Suit) -> u8 {
    match suit {
        crate::card::Suit::Spades => 0,
        crate::card::Suit::Hearts => 1,
        crate::card::Suit::Diamonds => 2,
        crate::card::Suit::Clubs => 3,
        crate::card::Suit::Joker => 4,
    }
}
