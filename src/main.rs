use macroquad::prelude::*;

#[macroquad::main("tic_tac_toe-mq-rs")]
async fn main() {
    let bg_color = macroquad::color::Color::from_rgba(255, 196, 196, 255);
    let board_texture = load_texture("./assests/board.png").await.unwrap();

    loop {
        clear_background(bg_color);
        draw_board(board_texture).await;
        next_frame().await;
    }
}

async fn draw_board(texture: Texture2D) {
    draw_texture_ex(texture,
        (screen_width() / 2.) - (screen_height() * 0.8) / 2.,
        screen_height() * 0.1,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(screen_height()*0.8, screen_height()*0.8)),
            ..DrawTextureParams::default()
        }
    )
}
