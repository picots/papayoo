use macroquad::prelude::*;

mod card;
mod deck;
mod game;
mod player;
mod render;

use game::GameState;
use render::{draw_game, hovered_hand_card};

fn window_conf() -> Conf {
    Conf {
        window_title: "Papayoo 🃏".to_string(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = game::Game::new();
    let mut ai_timer: f32 = 0.0; // Small delay before AI plays

    loop {
        let sw = screen_width();
        let sh = screen_height();
        let dt = get_frame_time();
        let (mx, my) = mouse_position();

        // --- Update timers ---
        game.update_timer(dt);

        // --- Hover detection for human hand ---
        let hand_size = game.players[0].hand.len();
        let hovered = if game.state == GameState::PlayerTurn {
            hovered_hand_card(sw, sh, hand_size, mx, my)
        } else {
            None
        };

        // --- Input handling ---
        if is_mouse_button_pressed(MouseButton::Left) {
            match game.state {
                GameState::PlayerTurn => {
                    if let Some(idx) = hovered {
                        game.human_play_card(idx);
                    }
                }
                _ => {}
            }
        }

        // --- AI turn with small delay ---
        if game.state == GameState::AITurn {
            ai_timer += dt;
            if ai_timer >= 0.6 {
                game.ai_play_card();
                ai_timer = 0.0;
            }
        } else {
            ai_timer = 0.0;
        }

        // --- Render ---
        draw_game(&game, hovered);

        next_frame().await;
    }
}
