use crate::{event::Permissions, user::User};

#[derive(Debug)]
pub struct Player {
    player: User,
}

impl Player {
    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::SubmitResultsUser, Permissions::DropSelf]
    }
}
#[derive(Debug)]
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
#[derive(Debug, Default)]
pub struct Organizer {
    organizer: User,
}

impl Organizer {
    fn permissions(&self) -> Vec<Permissions> {
        vec![Permissions::All]
    }
}

#[derive(Debug)]
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
