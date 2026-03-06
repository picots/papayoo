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
