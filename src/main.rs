use survive_rust::engine::GameEngine;
use survive_rust::ui::TerminalUi;

fn main() {
    let mut game = GameEngine::new();
    let ui = TerminalUi::new();

    ui.display_output("Welcome to Survive Rust!");
    ui.display_output("Type 'help' for commands, or 'quit' to exit.");

    while game.is_running() {
        let input = ui.read_command();
        let output = game.process_command(&input);
        ui.display_output(&output);
    }
}
