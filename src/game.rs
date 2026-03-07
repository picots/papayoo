use crate::card::{Card, Suit};
use crate::deck::Deck;
use crate::player::{Player, PlayerKind};

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    ChoosingPayoo, // First player of the round chooses Payoo suit
    PlayerTurn,    // Human picks a card to play
    AITurn,        // AI plays automatically
    TrickEnd,      // Show trick result briefly before next trick
    RoundEnd,      // Show round scores
    GameOver,      // Final scores
}

pub struct Game {
    pub players: Vec<Player>,
    pub state: GameState,
    pub current_player: usize,     // Index of the player whose turn it is
    pub trick_leader: usize,       // Who led this trick
    pub trick: Vec<(usize, Card)>, // (player_index, card)
    pub lead_suit: Option<Suit>,
    pub payoo_suit: Option<Suit>,
    pub round: u32,
    pub trick_winner: Option<usize>,
    pub state_timer: f32, // For timed transitions (AITurn, TrickEnd)
}

impl Game {
    pub fn new() -> Self {
        let players = vec![
            Player::new("Solal".to_string(), PlayerKind::Human),
            Player::new("Alice".to_string(), PlayerKind::AI),
            Player::new("Bob".to_string(), PlayerKind::AI),
            Player::new("Clara".to_string(), PlayerKind::AI),
        ];

        let mut game = Game {
            players,
            state: GameState::ChoosingPayoo,
            current_player: 0,
            trick_leader: 0,
            trick: Vec::new(),
            lead_suit: None,
            payoo_suit: None,
            round: 1,
            trick_winner: None,
            state_timer: 0.0,
        };

        game.deal_cards();
        game
    }

    fn deal_cards(&mut self) {
        let mut deck = Deck::new();
        let hands = deck.deal(4);
        for (player, hand) in self.players.iter_mut().zip(hands) {
            player.hand = hand;
        }
    }

    /// Human chooses the Payoo suit for this round.
    pub fn choose_payoo(&mut self, suit: Suit) {
        self.payoo_suit = Some(suit);
        self.state = if self.players[self.current_player].kind == PlayerKind::Human {
            GameState::PlayerTurn
        } else {
            GameState::AITurn
        };
    }

    /// Human plays a card from their hand (by index).
    pub fn human_play_card(&mut self, card_index: usize) {
        //let payoo = self.payoo_suit.clone().unwrap();
        let legal = self.players[0].legal_card_indices(self.lead_suit.as_ref());
        if !legal.contains(&card_index) {
            return; // Illegal move, ignore
        }

        let card = self.players[0].play_card(card_index);
        if self.lead_suit.is_none() {
            self.lead_suit = Some(card.suit.clone());
        }
        self.trick.push((0, card));
        self.advance_turn();
    }

    /// Advance to next player; if trick is complete, resolve it.
    pub fn advance_turn(&mut self) {
        let next = (self.current_player + 1) % 4;

        if self.trick.len() == 4 {
            self.resolve_trick();
        } else {
            self.current_player = next;
            self.state = if self.players[self.current_player].kind == PlayerKind::Human {
                GameState::PlayerTurn
            } else {
                GameState::AITurn
            };
        }
    }

    /// AI plays a card automatically.
    pub fn ai_play_card(&mut self) {
        let payoo = self.payoo_suit.clone().unwrap_or(Suit::Spades);
        let idx = self.players[self.current_player].ai_choose_card(self.lead_suit.as_ref(), &payoo);
        let card = self.players[self.current_player].play_card(idx);

        if self.lead_suit.is_none() {
            self.lead_suit = Some(card.suit.clone());
        }

        self.trick.push((self.current_player, card));
        self.advance_turn();
    }

    /// Determine who wins the trick (highest card of lead suit wins).
    fn resolve_trick(&mut self) {
        let lead = self.lead_suit.clone().unwrap();
        let winner_idx = self
            .trick
            .iter()
            .filter(|(_, c)| c.suit == lead)
            .max_by_key(|(_, c)| c.value)
            .map(|(i, _)| *i)
            .unwrap_or(self.trick_leader);

        // Give all trick cards to the winner
        let cards: Vec<Card> = self.trick.drain(..).map(|(_, c)| c).collect();
        self.players[winner_idx].tricks_taken.extend(cards);

        self.trick_winner = Some(winner_idx);
        self.lead_suit = None;

        // Check if round is over (all hands empty)
        if self.players[0].hand.is_empty() {
            self.end_round();
        } else {
            self.trick_leader = winner_idx;
            self.current_player = winner_idx;
            self.state = GameState::TrickEnd;
            self.state_timer = 1.5; // Show trick result for 1.5s
        }
    }

    pub fn after_trick_end(&mut self) {
        self.trick_winner = None;
        self.state = if self.players[self.current_player].kind == PlayerKind::Human {
            GameState::PlayerTurn
        } else {
            GameState::AITurn
        };
    }

    fn end_round(&mut self) {
        let payoo = self.payoo_suit.clone().unwrap_or(Suit::Spades);
        for player in &mut self.players {
            player.end_round(&payoo);
        }
        self.state = GameState::RoundEnd;
        self.state_timer = 3.0;
    }

    pub fn start_next_round(&mut self) {
        self.round += 1;
        // Check game end condition (e.g. 3 rounds played)
        if self.round > 3 {
            self.state = GameState::GameOver;
            return;
        }
        self.trick_leader = self.round as usize % 4;
        self.current_player = self.trick_leader;
        self.payoo_suit = None;
        self.lead_suit = None;
        self.trick.clear();
        self.deal_cards();
        self.state = GameState::ChoosingPayoo;
    }

    pub fn update_timer(&mut self, dt: f32) {
        if self.state_timer > 0.0 {
            self.state_timer -= dt;
            if self.state_timer <= 0.0 {
                match self.state {
                    GameState::TrickEnd => self.after_trick_end(),
                    GameState::RoundEnd => self.start_next_round(),
                    _ => {}
                }
            }
        }
    }
}
