#[allow(dead_code)]
use super::user::User;

enum EventType {
    Constructed(ConstructedEvent),
    Limited(LimitedEvent),
}

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

enum LimitedEvent {
    Draft,
    Sealed,
}
#[derive(Debug)]
struct Player {
    player: User,
}
struct Judge {
    judge: User,
}
struct Organizer {
    organizer: User,
}
struct ScoreKeeper {
    sk: User,
}

struct Event {
    type_: EventType,
    players: Vec<Player>,
    organizer: Organizer,
    score_keepers: Vec<ScoreKeeper>,
    judges: Vec<Judge>,
    start_time: String,
    rounds: i8,
    round_length: i8,
    current_round: i8,
    cut_to_top: i8, // 0 means no cut
}

impl Event {
    pub fn new(
        type_: EventType,
        players: Vec<Player>,
        organizer: Organizer,
        score_keepers: Vec<ScoreKeeper>,
        judges: Vec<Judge>,
        start_time: String,
        rounds: i8,
        round_length: i8,
        current_round: i8,
        cut_to_top: i8,
    ) -> Event {
        Event {
            type_,
            players,
            organizer,
            score_keepers,
            judges,
            start_time,
            rounds,
            round_length,
            current_round,
            cut_to_top,
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Event {
            type_: EventType::Constructed(ConstructedEvent::Standard),
            players: vec![],
            organizer: todo!(),
            score_keepers: todo!(),
            judges: todo!(),
            start_time: todo!(),
            rounds: todo!(),
            round_length: todo!(),
            current_round: todo!(),
            cut_to_top: todo!(),
        }
    }
}
