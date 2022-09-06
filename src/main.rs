mod models;
mod scenes;

#[macroquad::main("tic_tac_toe-mq-rs")]
async fn main() {
    // Loading a local game
    scenes::local_game::load_scene().await;

    println!("Thanks for playing all through...");
}