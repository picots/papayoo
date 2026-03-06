use crate::card::{Card, Suit};

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerKind {
    Human,
    AI,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub kind: PlayerKind,
    pub hand: Vec<Card>,
    pub score: u32,
    pub tricks_taken: Vec<Card>, // Cards collected this round
}

impl Player {
    pub fn new(name: String, kind: PlayerKind) -> Self {
        Self {
            name,
            kind,
            hand: Vec::new(),
            score: 0,
            tricks_taken: Vec::new(),
        }
    }

    /// Returns indices of cards that are legal to play given the lead suit.
    /// If no card matches the lead suit, all cards are legal.
    pub fn legal_card_indices(&self, lead_suit: Option<&Suit>) -> Vec<usize> {
        if let Some(lead) = lead_suit {
            let matching: Vec<usize> = self
                .hand
                .iter()
                .enumerate()
                .filter(|(_, c)| &c.suit == lead)
                .map(|(i, _)| i)
                .collect();

            if !matching.is_empty() {
                return matching;
            }
        }
        // No constraint: all cards are playable
        (0..self.hand.len()).collect()
    }

    /// Remove a card from hand by index and return it.
    pub fn play_card(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }

    /// Calculate round points from collected tricks.
    pub fn calculate_round_score(&self, payoo_suit: &Suit) -> u32 {
        self.tricks_taken.iter().map(|c| c.points(payoo_suit)).sum()
    }

    /// Clear tricks for the next round, add round score to total.
    pub fn end_round(&mut self, payoo_suit: &Suit) {
        self.score += self.calculate_round_score(payoo_suit);
        self.tricks_taken.clear();
    }

    /// Simple AI: prefer cards with 0 points; avoid Papayoo; follow suit.
    pub fn ai_choose_card(&self, lead_suit: Option<&Suit>, payoo_suit: &Suit) -> usize {
        let legal = self.legal_card_indices(lead_suit);

        // Try to play a card worth 0 points first
        if let Some(&idx) = legal
            .iter()
            .find(|&&i| self.hand[i].points(payoo_suit) == 0)
        {
            return idx;
        }

        // Otherwise play the card worth the fewest points (avoid Papayoo last)
        legal
            .into_iter()
            .min_by_key(|&i| self.hand[i].points(payoo_suit))
            .unwrap_or(0)
    }
}
