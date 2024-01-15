use serde::{Serialize, Deserialize};

use crate::{event::Permissions, user::User};

#[derive(Serialize, Deserialize)]
pub struct Player {
    player: User,
}

impl Player {
    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::SubmitResultsUser, Permissions::DropSelf]
    }
}
#[derive(Serialize, Deserialize)]
pub struct Judge {
    judge: User,
}

impl Judge {
    fn permissions(&self) -> Vec<Permissions> {
        vec![
            Permissions::SubmitResultsAny,
            Permissions::DropPlayers,
            Permissions::SubmitResultsAny,
            Permissions::StartRound,
        ]
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Organizer {
    organizer: User,
}

impl Organizer {
    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::All]
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScoreKeeper {
    sk: User,
}
impl ScoreKeeper {
    fn permissions(&self) -> Vec<Permissions> {
        vec![
            Permissions::AddDropPlayers,
            Permissions::SubmitResultsAny,
            Permissions::StartRound,
        ]
    }
}
