use macroquad::prelude::*;

pub mod models;
use models::*;

// Game Constants
const BG_COLOR: Color = color_u8!(255, 196, 196, 255);
const CELL_OUTLINE_COLOR: Color = color_u8!(238, 105, 131, 255);
const SELECTED_OUTLINE_COLOR: Color = color_u8!(17, 150, 124, 255);

#[macroquad::main("tic_tac_toe-mq-rs")]
async fn main() {
    // Loading textures
    let board_texture = load_texture("./assests/board.png").await.unwrap();
    let x_texture = load_texture("./assests/x.png").await.unwrap();
    let o_texture = load_texture("./assests/o.png").await.unwrap();
    board_texture.set_filter(FilterMode::Nearest);
 
    let mut board = Board::new();
    let mut selector = Place{row: Point::I, collum: Point::I};
    board.board_state = [[Cell::Filled(Player::X); 3]; 3];

    // Main loop
    loop {
        clear_background(BG_COLOR);
        interaction(&mut selector);
        draw_board(board.board_state, board_texture, x_texture, o_texture, &selector).await;
        next_frame().await;
    }
}

fn interaction(selector: &mut Place) {
    if is_key_pressed(KeyCode::W) {
        selector.up();
    } else if is_key_pressed(KeyCode::S) {
        selector.down();
    } else if is_key_pressed(KeyCode::A) {
        selector.left();
    } else if is_key_pressed(KeyCode::D) {
        selector.right();
    }
}

async fn draw_board(board_state: [[Cell; 3]; 3], tex1: Texture2D, tex2: Texture2D, tex3: Texture2D, selector: &Place) {
    // Dimentions
    let board_size = 64;
    let board_x = (screen_width() / 2.) - scale(board_size) / 2.;
    let board_y = screen_height() * 0.1;
    let outline_x = selector.row.value();
    let outline_y = selector.collum.value();

    // Drawing Cell Outlines
    // Drawing them before the board as full rectangles
    // because board backgournd texture have transparent holes for these outlines
    draw_rectangle(board_x, board_y, 
        scale(board_size), scale(board_size),
        CELL_OUTLINE_COLOR);
    draw_rectangle(
        board_x + scale(cell_start(outline_x)),
        board_y + scale(cell_start(outline_y)),
        scale(cell_outline_size(outline_x)),
        scale(cell_outline_size(outline_y)),
        SELECTED_OUTLINE_COLOR
    );

    // Drawing the board background
    draw_texture_ex(tex1,
        board_x, board_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(scale(board_size), scale(board_size))),
            ..DrawTextureParams::default()
        }
    );

    // Drawing cells
    for (i, row) in board_state.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Cell::Filled(player) = cell {
                let x = board_x + scale(cell_start(j) + 2);
                let y = board_y + scale(cell_start(i) + 2);
                let cell_tex = match player {
                    Player::X => tex2,
                    Player::O => tex3,
                };
                draw_texture_ex(cell_tex,
                    x, y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(
                            scale(cell_inner_size(j)),
                            scale(cell_inner_size(i))
                        )),
                        ..DrawTextureParams::default()
                    }
                );
            }
        }
    }
}

// Scales pixel values (relative to 64 x 64 grid) to screen resolution of the board
fn scale(relative: u32) -> f32 {
    (relative as f32 / 63.)* 0.8 * screen_height()
}

fn cell_start(index: usize) -> u32 {
    // Have to implement it like this
    // because the board beckground have different cell sizes
    match index {
        0 => 0,
        1 => 22,
        2 => 42,
        _ => 0
    }
}

fn cell_outline_size(index: usize) -> u32 {
    // Have to implement it like this
    // because the board beckground have different cell sizes
    match index {
        0 => 21,
        1 => 19,
        2 => 22,
        _ => 0
    }
}

fn cell_inner_size(index: usize) -> u32 {
    // Have to implement it like this
    // because the board beckground have different cell sizes
    match index {
        0 => 17,
        1 => 15,
        2 => 18,
        _ => 0
    }
}
