use serde::{Deserialize, Serialize};

/// Difficulty type
#[derive(Serialize, Deserialize)]
pub enum Difficulty {
    /// peaceful difficulty
    Peaceful = 0,
    /// easy difficulty
    Easy = 1,
    /// normal difficulty
    Normal = 2,
    /// hard difficulty
    Hard = 3,
}

/// Gamemode type
#[derive(Serialize, Deserialize)]
pub enum Gamemode {
    /// If no previous gamemode exists
    NoPreviousMode = -1,
    /// Survival mode
    Survival = 0,
    /// Creative mode
    Creative = 1,
    /// Adventure mode
    Adventure = 2,
    /// Spectator mode
    Spectator = 3,
}

/// LevelType type
#[derive(Serialize, Deserialize)]
pub enum LevelType {
    /// default world
    Default,
    /// flat world
    Flat,
    /// largeBiomes world
    LargeBiomes,
    /// amplified world
    Amplified,
    /// customized world
    Customized,
    /// buffet world
    Buffet,
    /// default_1_1 world
    Default11,
}

impl LevelType {
    /// used to convert enum value to String
    pub fn to_string(&self) -> String {
        match self {
            Self::Default => String::from("default"),
            Self::Flat => String::from("flat"),
            Self::LargeBiomes => String::from("largeBiomes"),
            Self::Amplified => String::from("amplified"),
            Self::Customized => String::from("customized"),
            Self::Buffet => String::from("buffet"),
            Self::Default11 => String::from("default_1_1"),
        }
    }
}
