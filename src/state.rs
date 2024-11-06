// src/state.rs
use bevy::prelude::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Resource, Default)]
pub struct GameState {
    pub is_universe_loaded: bool,
    pub input_universe_dimensions: (u32, u32),
}

#[derive(Serialize, Deserialize)]
pub struct UniverseData {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub created_at: String,
}

impl GameState {
    pub fn save_universe(&self) -> Result<UniverseData, String> {
        fs::create_dir_all("saves").map_err(|e| e.to_string())?;

        let timestamp = Utc::now().timestamp();
        let id = format!("universe_{}", timestamp);

        let universe_data = UniverseData {
            id: id.clone(),
            width: self.input_universe_dimensions.0,
            height: self.input_universe_dimensions.1,
            created_at: Utc::now().to_rfc3339(),
        };

        let file_path = PathBuf::from("saves").join(format!("{}.json", id));

        let json = serde_json::to_string_pretty(&universe_data).map_err(|e| e.to_string())?;

        fs::write(file_path, json).map_err(|e| e.to_string())?;

        Ok(universe_data)
    }
}

pub fn is_universe_loaded(game_state: Res<GameState>) -> bool {
    game_state.is_universe_loaded
}
