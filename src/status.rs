use crate::engine::GameEngine;

pub const PROVIDER_WEATHER: &str = "weather";
pub const PROVIDER_CAMP: &str = "camp";
pub const PROVIDER_PLAYER: &str = "player";

pub trait StatusProvider: std::fmt::Debug {
    fn name(&self) -> &'static str;
    fn status(&self, engine: &GameEngine) -> String;
}

#[derive(Debug)]
pub struct WeatherStatusProvider;
impl StatusProvider for WeatherStatusProvider {
    fn name(&self) -> &'static str {
        PROVIDER_WEATHER
    }
    fn status(&self, engine: &GameEngine) -> String {
        let hour = engine.time.hour;
        let weather_desc = if !(6..18).contains(&hour) {
            "The stars are out. It's cool outside. The plants look dark."
        } else {
            "The sun is high in the sky. It's quite warm outside. The plants look parched."
        };
        format!(
            "Day {}, {:02}:00. {} This feels like summer to you.",
            engine.time.day_count, hour, weather_desc
        )
    }
}

#[derive(Debug)]
pub struct CampStatusProvider;
impl StatusProvider for CampStatusProvider {
    fn name(&self) -> &'static str {
        PROVIDER_CAMP
    }
    fn status(&self, engine: &GameEngine) -> String {
        let wood = match engine.player.inventory.wood {
            0 => "No firewood",
            1..=49 => "A little firewood",
            50..=150 => "Some firewood",
            _ => "Lots of firewood",
        };
        let water = match engine.player.inventory.water {
            0 => "No water",
            1..=49 => "A little water",
            50..=150 => "Some water",
            _ => "Lots of water",
        };
        format!(
            "In the camp you see:\n ~ {}\n ~ {}\n ~ A sleeping spot",
            wood, water
        )
    }
}

#[derive(Debug)]
pub struct PlayerStatusProvider;
impl StatusProvider for PlayerStatusProvider {
    fn name(&self) -> &'static str {
        PROVIDER_PLAYER
    }
    fn status(&self, engine: &GameEngine) -> String {
        if engine.player.health >= 90 {
            "You feel rested and healthy.".to_string()
        } else if engine.player.health >= 50 {
            "You feel okay, but could use some rest.".to_string()
        } else {
            "You feel weak and injured.".to_string()
        }
    }
}
