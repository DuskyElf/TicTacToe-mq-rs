use macroquad::prelude::*;

pub mod models;
use models::*;

#[macroquad::main("tic_tac_toe-mq-rs")]
async fn main() {
    let bg_color = macroquad::color::Color::from_rgba(255, 196, 196, 255);

    // Loading textures
    let board_texture = load_texture("./assests/board.png").await.unwrap();
    let x_texture = load_texture("./assests/x.png").await.unwrap();
    let o_texture = load_texture("./assests/o.png").await.unwrap();
    board_texture.set_filter(FilterMode::Nearest);
 
    let mut board = Board::new();
    board.board_state = [[Cell::Filled(Player::X); 3]; 3];

    // Main loop
    loop {
        clear_background(bg_color);
        draw_board(board.board_state, board_texture, x_texture, o_texture).await;
        next_frame().await;
    }
}

async fn draw_board(board_state: [[Cell; 3]; 3], tex1: Texture2D, tex2: Texture2D, tex3: Texture2D) {
    // Calculating relative dimentions
    let board_x = (screen_width() / 2.) - (screen_height() * 0.8) / 2.;
    let board_y = screen_height() * 0.1;
    let board_size = screen_height() * 0.8;
    let padding = screen_height() * 0.0375;
    let cell_size = screen_height() * 0.1875;

    // Drawing the board
    draw_texture_ex(tex1,
        board_x, board_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(board_size, board_size)),
            ..DrawTextureParams::default()
        }
    );

    // Drawing cells
    for (i, row) in board_state.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Cell::Filled(player) = cell {
                let cell_tex = match player {
                    Player::X => tex2,
                    Player::O => tex3,
                };
                draw_texture_ex(cell_tex,
                    board_x + calc_cell_start(j) + padding,
                    board_y + calc_cell_start(i) + padding,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(cell_size, cell_size)),
                        ..DrawTextureParams::default()
                    }
                );
            }
        }
    }
}

fn calc_cell_start(index: usize) -> f32 {
    // Have to implement it like this
    // because the board background have different cell sizes
    if index <= 1 {
        screen_height() * 0.2625 * index as f32
    } else {
        screen_height() * 0.265 * index as f32
    }
}
