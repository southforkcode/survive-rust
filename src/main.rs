use survive_rust::engine::GameEngine;
use survive_rust::ui::TerminalUi;
use survive_rust::status::{CampStatusProvider, PlayerStatusProvider, WeatherStatusProvider};

fn main() {
    let mut game = GameEngine::new();
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
