mod models;
mod scenes;

#[macroquad::main("tic_tac_toe-mq-rs")]
async fn main() {
    let mut running = true;

    while running {
        // Loading a local game
        let game_result = scenes::local_game::load_scene().await;
        if let models::EndScreenResult::Quit = scenes::game_end::load_scene(game_result).await {
            running = false;
        }
    }

    println!("Thanks for playing all through...");
}