use anyhow::Context;
use serde::{de, Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use crate::objects::Card;

#[derive(Debug, Default, Serialize, Deserialize)]

pub struct Config {
    pub game: Game,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Game {
    pub shuffle: bool,
    pub deck_ids: Vec<i32>,
    #[serde(deserialize_with = "deserialize_cards")]
    pub cards: HashMap<i32, Card>,
}

fn deserialize_cards<'de, D>(deserializer: D) -> Result<HashMap<i32, Card>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let cards: Vec<Card> = de::Deserialize::deserialize(deserializer)?;
    let cards = cards.into_iter().map(|card| (card.id, card)).collect();
    Ok(cards)
}

impl Config {
    pub fn load(path: impl AsRef<Path> + Copy) -> anyhow::Result<Self> {
        let file = File::open(path).context(format!("File {:?} do not exist", path.as_ref()))?;
        let mut reader = BufReader::new(file);

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;

        let config = toml::from_str(&buffer)?;
        Ok(config)
    }
}
