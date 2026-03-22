use survive_rust::engine::GameEngine;
use survive_rust::status::{CampStatusProvider, PlayerStatusProvider, WeatherStatusProvider};
use survive_rust::ui::TerminalUi;

fn main() {
    let mut args = std::env::args().skip(1);
    let mut game = if let (Some(arg), Some(file)) = (args.next(), args.next()) {
        if arg == "-load" || arg == "--load" {
            GameEngine::load_from_file(&file).unwrap_or_else(|e| {
                println!("Failed to load game: {}. Starting a new game.", e);
                GameEngine::new()
            })
        } else {
            GameEngine::new()
        }
    } else {
        GameEngine::new()
    };
    game.register_status_provider(Box::new(WeatherStatusProvider));
    game.register_status_provider(Box::new(CampStatusProvider));
    game.register_status_provider(Box::new(PlayerStatusProvider));

    let ui = TerminalUi::new();

    ui.display_output("Welcome to Survive Rust!");
    ui.display_output("Type 'help' for commands, or 'quit' to exit.");

    while game.is_running() {
        let input = ui.read_command();
        let output = game.process_command(&input);
        ui.display_output(&output);
    }
}
