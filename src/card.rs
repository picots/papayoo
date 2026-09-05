use macroquad::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
    Joker,
}

impl Suit {
    /// Gives the suit symbol
    pub fn symbol(&self) -> &str {
        match self {
            Suit::Spades => "Pique",
            Suit::Hearts => "Coeur",
            Suit::Diamonds => "Carreau",
            Suit::Clubs => "Trèfle",
            Suit::Joker => "Atout",
        }
    }

    ///Gives the suit color
    pub fn color(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => RED,
            Suit::Joker => PURPLE,
            _ => BLACK,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: u8, // 1–10 for normal cards, 1–20 for Jokers
}

impl Card {
    /// Builds a card with a suit and a color
    pub fn new(suit: Suit, value: u8) -> Self {
        Card { suit, value }
    }

    /// Points that this card gives if collected in a trick.
    /// Depends on the current Payoo suit chosen each round.
    pub fn points(&self) -> u32 {
        match &self.suit {
            Suit::Joker => self.value as u32,
            s if self.is_papayoo(s) => 40, // The Papayoo!
            _ => 0,
        }
    }

    /// Checks if a suit is the payoo suit
    pub fn is_papayoo(&self, payoo_suit: &Suit) -> bool {
        &self.suit == payoo_suit && self.value == 7
    }
}

/// Gives the order to sort suits
pub fn suit_order(suit: &Suit) -> u8 {
    match suit {
        Suit::Spades => 0,
        Suit::Hearts => 1,
        Suit::Clubs => 2,
        Suit::Diamonds => 3,
        Suit::Joker => 4,
    }
}
