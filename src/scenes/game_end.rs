use macroquad::prelude::*;
use crate::models::*;

pub async fn load_scene(mut game_result: GameResult) -> EndScreenResult{
    // Loading textures
    let restart = load_texture("./assests/restart.png").await.unwrap();
    let quit = load_texture("./assests/quit.png").await.unwrap();
    restart.set_filter(FilterMode::Nearest);
    quit.set_filter(FilterMode::Nearest);

    // Menu Enviornment
    let mut selector = EndScreenResult::Restart;

    // Main Loop
    loop {
        clear_background(BG_COLOR);

        // Taking Player input, processing it and returning end screen result if any
        if let Some(selection) = interaction(&mut selector) {
            return selection;
        }
        draw(&mut game_result, &selector, restart, quit);
        next_frame().await;
    }
}

fn interaction(selector: &mut EndScreenResult) -> Option<EndScreenResult> {
    if is_key_pressed(KeyCode::W) {
        selector.up();
    } else if is_key_pressed(KeyCode::S) {
        selector.down();
    } else if is_key_pressed(KeyCode::Enter) {
        return Some(selector.clone());
    }

    None
}

fn draw(game_result: &GameResult, selector: &EndScreenResult, restart: Texture2D, quit: Texture2D) {
    let text_x = (screen_width() / 2.) - (screen_height() * 0.25);
    let text_y = screen_height() * 0.1;
    let text_size = screen_height() * 0.1;
    let board_size = screen_height() * 0.6;
    let board_x = (screen_width() / 2.) - board_size / 2.;
    let board_y = screen_height() * 0.3;

    draw_result_text(game_result, text_x, text_y, text_size);

    let texture = match selector {
        EndScreenResult::Restart => restart,
        EndScreenResult::Quit => quit,
    };

    draw_texture_ex(
        texture, board_x, board_y, BG_COLOR,
        DrawTextureParams {
            dest_size: Some(Vec2::new(board_size, board_size)),
            ..DrawTextureParams::default()
        }

    );
}

fn draw_result_text(game_result: &GameResult, x: f32, y: f32, s: f32) {
    let text = match game_result {
        GameResult::Draw => "It's a Draw".to_string(),
        GameResult::Won(winner) => format!("Player {} won", winner.print()),
    };

    draw_text(&text, x, y, s, CELL_OUTLINE_COLOR);
}