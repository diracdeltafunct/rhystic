use crate::roles::{Judge, Organizer, Player, ScoreKeeper};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
use super::user::User;

#[derive(Serialize, Deserialize)]
pub enum RulesEnforcement {
    Comp,
    Regular,
    Pro,
}
#[derive(Serialize, Deserialize)]
pub enum Permissions {
    AddDropPlayers,
    DropPlayers,
    DropSelf,
    EndEvent,
    StartRound,
    SubmitResultsAny,
    SubmitResultsUser,
    All,
    // ...
}

#[derive(Serialize, Deserialize)]
enum EventType {
    Constructed(ConstructedEvent),
    Limited(LimitedEvent),
}
#[derive(Serialize, Deserialize)]
enum ConstructedEvent {
    Standard,
    Modern,
    Legacy,
    Vintage,
    Commander,
    Brawl,
    Pauper,
    Pioneer,
    Historic,
    Penny,
    Gladiator,
    Oathbreaker,
    CanadianHighlander,
    DuelCommander,
    OldSchool93,
    Premodern,
    Frontier,
    TinyLeaders,
}
#[derive(Serialize, Deserialize)]
enum LimitedEvent {
    Draft,
    Sealed,
}
#[derive(Serialize, Deserialize)]
struct Event {
    type_: EventType,
    name: String,
    players: Vec<Player>,
    organizer: Organizer,
    score_keepers: Vec<ScoreKeeper>,
    judges: Vec<Judge>,
    start_time: DateTime<Utc>,
    rounds: i8,
    round_length: i8,
    current_round: i8,
    cut_to_top: Option<i8>,
    rules_enforcement: RulesEnforcement,
}
impl Event {
    pub fn new(
        type_: EventType,
        name: String,
        organizer: Organizer,
        start_time: DateTime<Utc>,
        round_length: i8,
        rules_enforcement: RulesEnforcement,
    ) -> Event {
        Event {
            type_,
            name,
            organizer,
            start_time,
            round_length,
            rules_enforcement,
            ..Event::default()
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            type_: EventType::Constructed(ConstructedEvent::Standard),
            name: String::from(""),
            players: vec![],
            organizer: Organizer::default(),
            score_keepers: vec![],
            judges: vec![],
            start_time: DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(0, 0), Utc),
            rounds: 3,
            round_length: 50,
            current_round: 0,
            cut_to_top: None,
            rules_enforcement: RulesEnforcement::Regular,
        }
    }
}
