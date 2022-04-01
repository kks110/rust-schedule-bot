use serenity::framework::standard::{Args};

pub struct DaysPlayable {
    pub game_id: String,
    pub weekdays: Vec<String>
}

impl DaysPlayable {
    pub fn new(game_id: String, weekdays: Vec<String>) -> DaysPlayable {
        DaysPlayable {
            game_id,
            weekdays
        }
    }
}

pub fn parse_day_registration(mut args: Args) -> Result<DaysPlayable, String>  {
    let game_id: String;
    let mut parsed_args = vec![];

    if let Ok(gid) = args.single::<String>() {
        game_id = gid
    } else {
        println!("Error with game id");
        return Err("Please enter a valid team name".to_string())
    }

    for _ in 0..(args.len() - 1) {
        if let Ok(day) = args.single::<String>() {
            parsed_args.push(day.to_lowercase())
        } else {
            println!("Error with days");
            return Err("Not enough days".to_string())
        }
    }

    Ok(DaysPlayable::new(game_id, parsed_args))
}