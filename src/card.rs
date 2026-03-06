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
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Joker => "🃏",
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
    pub value: u8, // 1–13 for normal cards, 1–8 for Jokers
}

impl Card {
    pub fn new(suit: Suit, value: u8) -> Self {
        Card { suit, value }
    }

    /// Points that this card gives if collected in a trick.
    /// Depends on the current Payoo suit chosen each round.
    pub fn points(&self, payoo_suit: &Suit) -> u32 {
        match &self.suit {
            Suit::Joker => self.value as u32, // Joker 1 = 1pt, Joker 8 = 8pt
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

    pub fn label(&self) -> String {
        match &self.suit {
            Suit::Joker => format!("J{}", self.value),
            _ => {
                let v = match self.value {
                    1 => "A".to_string(),
                    11 => "J".to_string(),
                    12 => "Q".to_string(),
                    13 => "K".to_string(),
                    n => n.to_string(),
                };
                format!("{}{}", v, self.suit.symbol())
            }
        }
    }

    pub fn is_papayoo(&self, payoo_suit: &Suit) -> bool {
        &self.suit == payoo_suit && self.value == 7
    }
}
