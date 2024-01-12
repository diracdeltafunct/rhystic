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

struct Event {
    type_: EventType,
    players: Vec<Player>,
    organizer: User,
    judges: Vec<User>,
    start_time: String,
    rounds: i8,
    round_length: i8,
    current_round: i8,
    cut_to_top: i8, // 0 means no cut

}
