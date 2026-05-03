use crate::engine::GameEngine;
use crate::engine::Resource;
use rand::prelude::*;

/// Standard event setup
pub enum Event {
    HiveAttack,
    AbandonedStash,
}

// Categorize Events
pub const GATHER_EVENTS: [Event; 2] = [Event::AbandonedStash, Event::AbandonedStash];

impl Event {
    pub fn execute(&self, engine: &mut GameEngine) -> String {
        match self {
            Event::HiveAttack => {
                engine.player.health -= 5;
                "You've bumped into a bee hive filled with angry bees. They begin stining you! (-5 health)".to_string()
            }

            Event::AbandonedStash  => {
                let resource: Resource = random_resource();
                match resource {
                    Resource::Food => {
                        engine.player.inventory.food += 5;
                        return "Your treck in search of resources led you to an abandoned stash of food (+5 lb.)".to_owned();
                    }
                    Resource::Water => {
                        engine.player.inventory.water += 50;
                        return "While in search of resources, you stumble upon a creek flowing with fresh water (+15 L)".to_owned();
                    }
                    _ => {}
                }

                "".to_owned()
            }
        }
    }
}

fn random_resource() -> Resource {
    const RESOURCES: &[Resource] = &[Resource::Wood, Resource::Water];
    let mut rng = rand::rng();
    *RESOURCES.choose(&mut rng).unwrap_or(&Resource::Unknown)
}