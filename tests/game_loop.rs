use cucumber::{World, given, then, when};
use survive_rust::engine::GameEngine;

#[derive(Debug, Default, World)]
pub struct GameWorld {
    engine: Option<GameEngine>,
    last_output: String,
}

#[given("the game is running")]
fn game_is_running(world: &mut GameWorld) {
    use survive_rust::status::{CampStatusProvider, PlayerStatusProvider, WeatherStatusProvider};
    let mut engine = GameEngine::new();
    engine.register_status_provider(Box::new(WeatherStatusProvider));
    engine.register_status_provider(Box::new(CampStatusProvider));
    engine.register_status_provider(Box::new(PlayerStatusProvider));
    world.engine = Some(engine);
}

#[when(expr = "I type the command {string}")]
fn type_command(world: &mut GameWorld, command: String) {
    if let Some(engine) = world.engine.as_mut() {
        world.last_output = engine.process_command(&command);
    }
}

#[then(expr = "the output should contain {string}")]
fn output_contains(world: &mut GameWorld, expected: String) {
    assert!(
        world.last_output.contains(&expected),
        "Expected output containing '{}', found '{}'",
        expected,
        world.last_output
    );
}

#[then("the game should still be running")]
fn game_still_running(world: &mut GameWorld) {
    if let Some(engine) = &world.engine {
        assert!(engine.is_running(), "Expected game to be running");
    } else {
        panic!("Engine not initialized");
    }
}

#[then("the game should not be running")]
fn game_not_running(world: &mut GameWorld) {
    if let Some(engine) = &world.engine {
        assert!(!engine.is_running(), "Expected game to NOT be running");
    } else {
        panic!("Engine not initialized");
    }
}

#[tokio::main]
async fn main() {
    GameWorld::run("tests/features").await;
}
