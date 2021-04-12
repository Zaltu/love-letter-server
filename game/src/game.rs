use std::collections::HashMap;

use log::info;
use rand::seq::SliceRandom;

use crate::{
    config::Config,
    error::ServerError,
    objects::{Action, Card, Player},
};

#[derive(Debug, Default)]
pub struct Game {
    config: Config,

    current_deck: Vec<Card>,
    players: Vec<Player>,

    actions: HashMap<i32, Vec<Action>>,
}

impl Game {
    pub fn new(config: Config) -> Self {
        Game {
            config,
            ..Default::default()
        }
    }

    /// Initial setup before any game.
    /// Shuffles a new deck.
    pub fn setup(&mut self) -> Result<(), ServerError> {
        let mut rng = rand::thread_rng();

        let mut deck_ids = self.config.game.deck_ids.clone();

        // Shuffle the deck ids
        if self.config.game.shuffle {
            deck_ids.shuffle(&mut rng);
        }

        // Create the new deck for the new game
        self.current_deck = deck_ids
            .iter()
            .map(|id| {
                self.config
                    .game
                    .cards
                    .get(id)
                    .ok_or_else(|| ServerError(format!("Card Id {} do not exist", id)))
                    .map(|card| card.clone())
            })
            .collect::<Result<_, _>>()?;

        Ok(())
    }

    pub fn new_game(&mut self) -> Result<(), ServerError> {
        for id in self
            .players
            .iter()
            .map(|player| player.id)
            .collect::<Vec<_>>()
        {
            let action = self.draw();

            match action {
                Action::DrawCard(_) => {
                    self.actions.insert(id, vec![action]);
                }
                Action::EndRound => unimplemented!("no enough card at new game"),
                // _ => unreachable!("unexpected action at new game"),
            }
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Action {
        if let Some(card) = self.current_deck.pop() {
            Action::DrawCard(card)
        } else {
            info!("round ended");
            Action::EndRound
        }
    }

    pub fn add_player(&mut self) -> Result<i32, ServerError> {
        if self.players.len() < 4 {
            let id = self.players.len() as i32;
            self.players.push(Player {
                id,
                ..Default::default()
            });
            Ok(id)
        } else {
            Err(ServerError(
                "Server only supports up to 4 players".to_string(),
            ))
        }
    }
}
