use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Player {
    pub id: i32,
    pub tokens_of_affection: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Action {
    DrawCard(Card),
    DiscardCard
    
    ,
    KnockOut,
    EndRound,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Answer {
    CardDiscarded(i32),
    PlayerChoosen(i32),
}