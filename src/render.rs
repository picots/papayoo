use macroquad::prelude::*;

use crate::card::Card;
use crate::game::{Game, GameState};

// Card dimensions
pub const CARD_W: f32 = 70.0;
pub const CARD_H: f32 = 100.0;
const CARD_RADIUS: f32 = 6.0;

pub fn draw_card(card: &Card, x: f32, y: f32, highlighted: bool, grayed: bool) {
    let bg = if grayed {
        Color::new(0.75, 0.75, 0.75, 1.0)
    } else if highlighted {
        Color::new(1.0, 1.0, 0.7, 1.0)
    } else {
        WHITE
    };

    draw_rounded_rect(x, y, CARD_W, CARD_H, CARD_RADIUS, bg);
    draw_rounded_rect_outline(
        x,
        y,
        CARD_W,
        CARD_H,
        CARD_RADIUS,
        if highlighted { GOLD } else { DARKGRAY },
    );

    let sym = card.suit.symbol();
    let color = card.suit.color();
    draw_text(sym, x + 6.0, y + 20.0, 20.0, color);

    let value = card.value;
    draw_text(
        &value.to_string(),
        x + CARD_W / 2.0 - 10.0,
        y + CARD_H / 2.0 + 8.0,
        28.0,
        color,
    );
}

pub fn draw_card_back(x: f32, y: f32) {
    draw_rounded_rect(x, y, CARD_W, CARD_H, CARD_RADIUS, DARKBLUE);
    draw_rounded_rect_outline(
        x,
        y,
        CARD_W,
        CARD_H,
        CARD_RADIUS,
        Color::new(0.0, 0.0, 0.5, 1.0),
    );
}

fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, _r: f32, color: Color) {
    draw_rectangle(x, y, w, h, color);
}

fn draw_rounded_rect_outline(x: f32, y: f32, w: f32, h: f32, _r: f32, color: Color) {
    draw_rectangle_lines(x, y, w, h, 2.0, color);
}

pub fn draw_game(game: &Game, hovered_card: Option<usize>) {
    let sw = screen_width();
    let sh = screen_height();

    // Background (green felt)
    clear_background(Color::new(0.13, 0.55, 0.13, 1.0));

    // Title + Round info
    draw_text(
        &format!("PAPAYOO - Manche {}", game.round),
        sw / 2.0 - 120.0,
        30.0,
        28.0,
        WHITE,
    );

    // Payoo suit indicator
    if let Some(payoo) = &game.payoo_suit {
        let txt = format!("Payoo : {}", payoo.symbol());
        draw_text(&txt, sw - 160.0, 30.0, 24.0, YELLOW);
    }

    // Scores (top-left)
    for (i, p) in game.players.iter().enumerate() {
        let y = 60.0 + i as f32 * 24.0;
        let active = i == game.current_player;
        let color = if active { YELLOW } else { WHITE };
        draw_text(
            &format!("{}: {} pts", p.name, p.score),
            12.0,
            y,
            20.0,
            color,
        );
    }

    // Trick center — show last_trick during TrickEnd, otherwise current trick
    let displayed_trick = if game.state == GameState::TrickEnd {
        &game.last_trick
    } else {
        &game.trick
    };
    let trick_x_start = sw / 2.0 - (displayed_trick.len() as f32 * (CARD_W + 8.0)) / 2.0;
    for (i, (player_idx, card)) in displayed_trick.iter().enumerate() {
        let x = trick_x_start + i as f32 * (CARD_W + 8.0);
        let y = sh / 2.0 - CARD_H / 2.0;
        draw_card(card, x, y, false, false);
        draw_text(&game.players[*player_idx].name, x, y - 16.0, 16.0, WHITE);
    }

    // Opponent hands (top — show backs)
    let opp_positions = [
        (sw / 2.0 - 40.0, 75.0),       // Top (player 2)
        (30.0, sh / 2.0 - 50.0),       // Left (player 1)
        (sw - 180.0, sh / 2.0 - 50.0), // Right (player 3)
    ];
    for (i, (bx, by)) in opp_positions.iter().enumerate() {
        let p = &game.players[i + 1];
        let count = p.hand.len();
        draw_text(&p.name, *bx, *by - 4.0, 16.0, WHITE);
        for j in 0..count.min(10) {
            draw_card_back(*bx + j as f32 * 8.0, *by);
        }
        if count > 10 {
            draw_text(
                &format!("+{}", count - 10),
                *bx + 88.0,
                *by + 50.0,
                16.0,
                WHITE,
            );
        }
    }

    // Human hand (bottom)
    let human = &game.players[0];
    let hand = &human.hand;
    let hand_w = hand.len() as f32 * (CARD_W + 6.0);
    let hx_start = sw / 2.0 - hand_w / 2.0;
    let hy = sh - CARD_H - 24.0;

    let legal = match game.state {
        GameState::PlayerTurn => human.legal_card_indices(game.lead_suit.as_ref()),
        _ => vec![],
    };

    for (i, card) in hand.iter().enumerate() {
        let x = hx_start + i as f32 * (CARD_W + 6.0);
        let is_hovered = hovered_card == Some(i);
        let y = if is_hovered { hy - 12.0 } else { hy };
        let is_legal = legal.contains(&i);
        let grayed = game.state == GameState::PlayerTurn && !is_legal;
        draw_card(card, x, y, is_hovered && is_legal, grayed);
    }

    // State overlays
    match &game.state {
        GameState::TrickEnd => {
            if let Some(w) = game.trick_winner {
                let msg = format!("{} remporte le pli !", game.players[w].name);
                draw_centered_message(&msg, sw, sh);
            }
        }
        GameState::RoundEnd => draw_round_end_overlay(game, sw, sh),
        GameState::GameOver => draw_game_over_overlay(game, sw, sh),
        GameState::AITurn => {
            draw_text(
                "L'IA réfléchit...",
                sw / 2.0 - 80.0,
                sh / 2.0 - 80.0,
                22.0,
                LIGHTGRAY,
            );
        }
        _ => {}
    }
}

pub fn hovered_hand_card(sw: f32, sh: f32, hand_size: usize, mx: f32, my: f32) -> Option<usize> {
    let hand_w = hand_size as f32 * (CARD_W + 6.0);
    let hx_start = sw / 2.0 - hand_w / 2.0;
    let hy = sh - CARD_H - 24.0;
    for i in 0..hand_size {
        let x = hx_start + i as f32 * (CARD_W + 6.0);
        if mx >= x && mx <= x + CARD_W && my >= hy - 12.0 && my <= hy + CARD_H {
            return Some(i);
        }
    }
    None
}

fn draw_round_end_overlay(game: &Game, sw: f32, sh: f32) {
    draw_rectangle(
        sw / 2.0 - 200.0,
        sh / 2.0 - 120.0,
        400.0,
        240.0,
        Color::new(0.0, 0.0, 0.0, 0.85),
    );
    draw_text(
        "Fin de la manche !",
        sw / 2.0 - 120.0,
        sh / 2.0 - 90.0,
        26.0,
        YELLOW,
    );
    for (i, p) in game.players.iter().enumerate() {
        draw_text(
            &format!("{}: {} pts", p.name, p.score),
            sw / 2.0 - 100.0,
            sh / 2.0 - 50.0 + i as f32 * 28.0,
            22.0,
            WHITE,
        );
    }
}

fn draw_game_over_overlay(game: &Game, sw: f32, sh: f32) {
    draw_rectangle(
        sw / 2.0 - 220.0,
        sh / 2.0 - 140.0,
        440.0,
        280.0,
        Color::new(0.0, 0.0, 0.0, 0.9),
    );
    draw_text(
        "*** FIN DE PARTIE ***",
        sw / 2.0 - 145.0,
        sh / 2.0 - 110.0,
        30.0,
        GOLD,
    );

    let mut scores: Vec<(String, u32)> = game
        .players
        .iter()
        .map(|p| (p.name.clone(), p.score))
        .collect();
    scores.sort_by_key(|(_, s)| *s);

    for (i, (name, score)) in scores.iter().enumerate() {
        let medal = match i {
            0 => "1.",
            1 => "2.",
            2 => "3.",
            _ => "4.",
        };
        draw_text(
            &format!("{} {} — {} pts", medal, name, score),
            sw / 2.0 - 150.0,
            sh / 2.0 - 60.0 + i as f32 * 36.0,
            24.0,
            WHITE,
        );
    }

    draw_text(
        "Fermez la fenêtre pour quitter.",
        sw / 2.0 - 160.0,
        sh / 2.0 + 110.0,
        18.0,
        LIGHTGRAY,
    );
}

fn draw_centered_message(msg: &str, sw: f32, sh: f32) {
    let w = msg.len() as f32 * 11.0;
    draw_rectangle(
        sw / 2.0 - w / 2.0 - 16.0,
        sh / 2.0 - 22.0,
        w + 32.0,
        36.0,
        Color::new(0.0, 0.0, 0.0, 0.75),
    );
    draw_text(msg, sw / 2.0 - w / 2.0, sh / 2.0 + 4.0, 24.0, YELLOW);
}
