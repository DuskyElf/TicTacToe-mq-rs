use macroquad::prelude::*;
use crate::models::*;

// Game Constants
const BG_COLOR: Color = color_u8!(255, 196, 196, 255);
const CELL_OUTLINE_COLOR: Color = color_u8!(238, 105, 131, 255);
const SELECTED_OUTLINE_COLOR: Color = color_u8!(17, 150, 124, 255);

pub async fn load_scene() -> GameResult {
    // Loading textures
    let board_texture = load_texture("./assests/board.png").await.unwrap();
    let x_texture = load_texture("./assests/x.png").await.unwrap();
    let o_texture = load_texture("./assests/o.png").await.unwrap();
    board_texture.set_filter(FilterMode::Nearest);
 
    // Game Enviornment
    let mut game = Game {
        board: Board::new(),
        selector: Place{row: Point::I, collum: Point::I},
        current_turn: Player::X,
    };

    // Main loop
    loop {
        clear_background(BG_COLOR);

        // Taking player input, processing it and returning game result if any
        if let Some(game_result) = interaction(&mut game) {
            return game_result;
        }
        draw(&game, board_texture, x_texture, o_texture);
        next_frame().await;
    }
}

fn interaction(game: &mut Game) -> Option<GameResult> {
    if is_key_pressed(KeyCode::W) {
        game.selector.up();
    } else if is_key_pressed(KeyCode::S) {
        game.selector.down();
    } else if is_key_pressed(KeyCode::A) {
        game.selector.left();
    } else if is_key_pressed(KeyCode::D) {
        game.selector.right();
    } else if is_key_pressed(KeyCode::Space) {
        if let Some(game_result) = game_logic(game) {
            return Some(game_result);
        }
    }

    None
}

fn game_logic(game: &mut Game) -> Option<GameResult>{
    // Canceling the move, if the place is already filled
    if let Cell::Filled(_) = game.board[&game.selector] {
        return None;
    }

    // Changing the board to display the move
    game.board.play_move(&game.selector, &game.current_turn);

    game.current_turn = match game.current_turn {
        Player::X => Player::O,
        Player::O => Player::X,
    };

    // Checking if someone won
    let b = game.board.board_state;
    for i in 0..3 {
        // Horizontal Checks
        if let Cell::Filled(player) = b[i][0] {
            if b[i][0] == b[i][1] && b[i][1] == b[i][2] {
                return Some(GameResult::Won(player));
            }
        }
        // Vertical Checks
        if let Cell::Filled(player) = b[0][i] {
            if b[0][i] == b[1][i] && b[1][i] == b[2][i] {
                return Some(GameResult::Won(player));
            }
        }
    }
    // Diagonal Checks
    if let Cell::Filled(player) = b[0][0] {
        if b[0][0] == b[1][1] && b[1][1] == b[2][2] {
            return Some(GameResult::Won(player));
        }
    }
    if let Cell::Filled(player) = b[0][2] {
        if b[2][0] == b[1][1] && b[1][1] == b[0][2] {
            return Some(GameResult::Won(player));
        }
    }

    // Checking if the game was tied
    if check_tie(&game.board) {
        return Some(GameResult::Draw);
    }

    None
}

fn check_tie(board: &Board) -> bool {
    for i in board.board_state {
        for cell in i {
            if cell == Cell::Empty {
                return false;
            }
        }
    }

    true
}

fn draw(game: &Game, tex1: Texture2D, tex2: Texture2D, tex3: Texture2D) {
    // Dimentions
    let board_size = 64;
    let board_x = (screen_width() / 2.) - scale(board_size) / 2.;
    let board_y = screen_height() * 0.2;
    let turn_text_x = (screen_width() / 2.) - (screen_height() * 0.17);
    let turn_text_y = screen_height() * 0.1;
    let turn_text_size = screen_height() * 0.1;

    // Displaying the current
    draw_turn_text(&game.current_turn, turn_text_x, turn_text_y, turn_text_size);

    // Drawing Cell Outlines
    // Drawing them before the board as full rectangles
    // because board background texture have transparent holes for these outlines
    draw_cell_outlines(&game.selector, board_x, board_y, board_size);
    
    draw_board(board_x, board_y, board_size, game.board.board_state, tex1, tex2, tex3);
}

fn draw_turn_text(current_turn: &Player, x: f32, y: f32, s: f32) {
    draw_text(
        &format!("Turn: {}", current_turn.print()),
        x, y, s, CELL_OUTLINE_COLOR
    );
}

fn draw_cell_outlines(selector: &Place, board_x: f32, board_y: f32, board_size:u32) {
    let outline_x = selector.collum.value();
    let outline_y = selector.row.value();

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
}

fn draw_board(board_x: f32, board_y: f32, board_size:u32, board_state: [[Cell; 3]; 3], tex1: Texture2D, tex2: Texture2D, tex3: Texture2D) {
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
    (relative as f32 / 63.)* 0.7 * screen_height()
}

fn cell_start(index: usize) -> u32 {
    // Have to implement it like this
    // because the board beckground have different cell sizes
    match index {
        0 => 0,
        1 => 23,
        2 => 42,
        _ => 0
    }
}

fn cell_outline_size(index: usize) -> u32 {
    // Have to implement it like this
    // because the board beckground have different cell sizes
    match index {
        0 => 22,
        1 => 18,
        2 => 22,
        _ => 0
    }
}

fn cell_inner_size(index: usize) -> u32 {
    // Have to implement it like this
    // because the board beckground have different cell sizes
    match index {
        0 => 18,
        1 => 14,
        2 => 18,
        _ => 0
    }
}