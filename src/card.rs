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
    pub fn symbol(&self) -> &str {
        match self {
            Suit::Spades => "Pique",
            Suit::Hearts => "Coeur",
            Suit::Diamonds => "Carreau",
            Suit::Clubs => "Trèfle",
            Suit::Joker => "Atout",
        }
    }

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
    pub fn new(suit: Suit, value: u8) -> Self {
        Card { suit, value }
    }

    /// Points that this card gives if collected in a trick.
    /// Depends on the current Payoo suit chosen each round.
    pub fn points(&self, payoo_suit: &Suit) -> u32 {
        match &self.suit {
            Suit::Joker => self.value as u32,
            s if s == payoo_suit => {
                if self.value == 7 {
                    40 // The Papayoo!
                } else {
                    self.value as u32
                }
            }
            _ => 0,
        }
    }

    #[allow(dead_code)]
    pub fn is_papayoo(&self, payoo_suit: &Suit) -> bool {
        &self.suit == payoo_suit && self.value == 7
    }
}
