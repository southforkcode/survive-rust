use cucumber::{World, given, then, when};
use survive_rust::events;
use survive_rust::engine::GameEngine;

#[derive(Debug, Default, World)]
pub struct GameWorld {
    engine: Option<GameEngine>,
    last_output: String,
    player_before: Option<survive_rust::engine::Player>
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

#[when("an AbandonedStash event is executed")]
fn abandoned_stash_event(world: &mut GameWorld) {
    if let Some(engine) = world.engine.as_mut() {
        world.player_before = Some(engine.player.clone());
        world.last_output = engine.execute_event(&events::Event::AbandonedStash);
    }
}

#[then("either food or water inventory should increase")]
fn inventory_increase(world: &mut GameWorld) {
    if let Some(engine) = world.engine.as_mut() {
        if let Some(player_before) = world.player_before.as_mut() {
            if world.last_output.contains("water") {
                assert!(engine.player.inventory.water > player_before.inventory.water)
            } else if world.last_output.contains("food") {
                assert!(engine.player.inventory.food > player_before.inventory.food)
            }
        }
    }
}

#[when("a HiveAttack event is executed")]
fn hive_attack_event(world: &mut GameWorld) {
    if let Some(engine) = world.engine.as_mut() {
        world.player_before = Some(engine.player.clone());
        world.last_output = engine.execute_event(&events::Event::HiveAttack);
    }
}

#[then("the player's health should decrease")]
fn health_decreased(world: &mut GameWorld) {
    if let Some(engine) = world.engine.as_mut() {
        if let Some(player_before) = world.player_before.as_mut() {
            assert!(engine.player.health < player_before.health);
        } 
    }
}

#[tokio::main]
async fn main() {
    GameWorld::run("tests/features").await;
}
