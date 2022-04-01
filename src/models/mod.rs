use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

#[derive(Serialize, Deserialize)]
pub struct PlayerAndDays {
    pub player_name: String,
    pub weekdays: Vec<Days>
}

impl PlayerAndDays {
    fn new(player_name: String, weekdays: Vec<String>) -> PlayerAndDays {
        let mut days = vec![];

        for day in weekdays {
            match &day[..] {
                "mon" => days.push(Days::Monday),
                "tue" => days.push(Days::Tuesday),
                "wed" => days.push(Days::Wednesday),
                "thu" => days.push(Days::Thursday),
                "fri" => days.push(Days::Friday),
                "sat" => days.push(Days::Saturday),
                "sun" => days.push(Days::Sunday),
                _ => {}
            }
        }

        PlayerAndDays {
            player_name,
            weekdays: days
        }

    }
}

#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub game_id: String,
    pub players: Vec<PlayerAndDays>
}

impl GameData {
    pub fn new(game_id: String, player: String, days: Vec<String>) -> GameData {
        let player_and_days = PlayerAndDays::new(player, days);

        GameData {
            game_id,
            players: vec![player_and_days]
        }
    }
}