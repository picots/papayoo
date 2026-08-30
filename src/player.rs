use crate::card::{Card, Suit};

use rand::prelude::SliceRandom;

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
    pub fn calculate_round_score(&self) -> u32 {
        self.tricks_taken.iter().map(|c| c.points()).sum()
    }

    /// Clear tricks for the next round, add round score to total.
    pub fn end_round(&mut self) {
        self.score += self.calculate_round_score();
        self.tricks_taken.clear();
    }

    /// Simple AI: prefer cards with 0 points; avoid Papayoo; follow suit.
    pub fn ai_choose_card(&self, lead_suit: Option<&Suit>) -> usize {
        let legal = self.legal_card_indices(lead_suit);

        // Try to play a card worth 0 points first
        if let Some(&idx) = legal.iter().find(|&&i| self.hand[i].points() == 0) {
            return idx;
        }

        // Otherwise play the card worth the fewest points (avoid Papayoo last)
        legal
            .into_iter()
            .min_by_key(|&i| self.hand[i].points())
            .unwrap_or(0)
    }

    pub fn ai_cards_to_give(&self) -> Vec<Card> {
        let mut hand = self.hand.clone();
        hand.shuffle(&mut rand::thread_rng());
        let mut cards_to_give = Vec::new();
        for card in &hand {
            if card.points() == 0 {
                cards_to_give.push(card.clone());
            }
            if cards_to_give.len() == 5 {
                return cards_to_give;
            }
        }
        cards_to_give
    }

    pub fn sort_hand(&mut self) {
        self.hand.sort_by(|a, b| {
            let sa = suit_order(&a.suit);
            let sb = suit_order(&b.suit);
            sa.cmp(&sb).then(a.value.cmp(&b.value))
        });
    }
}

pub fn suit_order(suit: &crate::card::Suit) -> u8 {
    match suit {
        crate::card::Suit::Spades => 0,
        crate::card::Suit::Hearts => 1,
        crate::card::Suit::Clubs => 2,
        crate::card::Suit::Diamonds => 3,
        crate::card::Suit::Joker => 4,
    }
}
