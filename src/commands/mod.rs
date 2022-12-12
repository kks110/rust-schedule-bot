use std::fmt::{Display, Formatter};
use crate::{
    game_code,
    messages,
    models::{Game, User},
    database,
    Context,
    Error
};

#[poise::command(slash_command)]
pub async fn new_game(
    ctx: Context<'_>,
    #[description = "Game name"]
    game_name: String,
) -> Result<(), Error> {
    let game_code = game_code::generate();

    let mut error_message: Option<String> = None;

    let conn = database::establish_connection();
    match database::games::create_game(&conn, &game_code, &game_name, 5) {
        Ok(_) => {}
        Err(e) => { error_message = Some(e.to_string()) }
    };

    if let Some(error_message) = error_message {
        let title = format!("An Error occurred: {}", error_message);
        messages::send_error_message(ctx, title).await?;
    }

    let title = format!("Game Created: {}", game_name);
    let description = format!("Please log your days using the game ID: **{}**", game_code);

    messages::send_message(ctx, title, description).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn games(ctx: Context<'_>) -> Result<(), Error> {
    let mut error_message: Option<String> = None;
    let mut games: Option<Vec<Game>> = None;

    let conn = database::establish_connection();
    match database::games::load_games(&conn) {
        Ok(g) => { games = Some(g) }
        Err(e) => { error_message = Some(e.to_string()) }
    };

    let mut description = "".to_string();
    let title = "Game List:".to_string();

    if let Some(games) = games {
        for game in games {
            match database::users::load_user_count_by_game_id(&conn, game.id) {
                Ok(user_count) => {
                    description.push_str(&format!("{} ({}) - {}/{} players registered\n", game.name, game.code, user_count, game.user_count));
                }
                Err(e) => { error_message = Some(e.to_string()) }
            }
        }
    }

    if let Some(error_message) = error_message {
        messages::send_error_message(ctx, error_message).await?;
    }

    messages::send_message(ctx, title, description).await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn delete_game(
    ctx: Context<'_>,
    #[description = "Game code"]
    game_code: String,
) -> Result<(), Error> {
    let mut error_message: Option<String> = None;

    let conn = database::establish_connection();

    match database::games::delete_game(&conn, &game_code) {
        Ok(_) => {}
        Err(e) => { error_message = Some(format!("Error deleting game: {}", e)) }
    };


    if let Some(error_message) = error_message {
        messages::send_error_message(ctx,error_message).await?;
    }

    messages::send_message(ctx, "Game delete successfully", "").await?;

    Ok(())
}

#[poise::command(slash_command)]
pub async fn register_for_game(
    ctx: Context<'_>,
    #[description = "Game code"]
    game_code: String,
    #[description = "Monday"]
    #[lazy]
    monday: Option<bool>,
    #[description = "Tuesday"]
    #[lazy]
    tuesday: Option<bool>,
    #[description = "Wednesday"]
    #[lazy]
    wednesday: Option<bool>,
    #[description = "Thursday"]
    #[lazy]
    thursday: Option<bool>,
    #[description = "Friday"]
    #[lazy]
    friday: Option<bool>,
) -> Result<(), Error> {
    let mut days_playable: Vec<String> = vec![];
    if let Some(mon) = monday {
        if mon {
            days_playable.push( "monday".to_string())
        }
    }
    if let Some(tue) = tuesday {
        if tue {
            days_playable.push("tuesday".to_string())
        }
    }
    if let Some(weds) = wednesday {
        if weds {
            days_playable.push("wednesday".to_string())
        }
    }
    if let Some(thurs) = thursday {
        if thurs {
            days_playable.push("thursday".to_string())
        }
    }
    if let Some(fri) = friday {
        if fri {
            days_playable.push("friday".to_string())
        }
    }

    let player: &str = &ctx.author().name;

    let mut game: Option<Game> = None;
    let mut user_and_game: Option<(User, Game)> = None;
    let mut error_message: Option<String> = None;

    let conn = database::establish_connection();
    match database::games::load_game_by_code(&conn, &game_code) {
        Ok(g) => { game = Some(g) }
        Err(_) => { error_message = Some(format!("Could not find game code: {}", &game_code)) }
    };

    if let Some(game) = game {
        match database::users::update_or_create(&conn, player, game.id, days_playable) {
            Ok(u) => { user_and_game = Some((u, game)) }
            Err(e) => { error_message = Some(format!("Whilst registering for the game the following error occurred: {}", e)) }
        };
    };

    if let Some(error_message) = error_message {
        messages::send_error_message(ctx, error_message).await?;
    }

    if let Some((u, game)) = user_and_game {
        let title = format!("{}'s available days for game {}. (Code: {})", u.name, game.name, game.code);
        let description = format!(
            "{} Monday \n {} Tuesday \n {} Wednesday \n {} Thursday \n {} Friday \n {} Saturday \n {} Sunday \n",
            available(u.monday),
            available(u.tuesday),
            available(u.wednesday),
            available(u.thursday),
            available(u.friday),
            available(u.saturday),
            available(u.sunday),
        );

        messages::send_message(ctx, title, description).await?;
    }

    Ok(())
}


#[poise::command(slash_command)]
pub async fn availability(
    ctx: Context<'_>,
    #[description = "Game code"]
    game_code: String,
) -> Result<(), Error> {
    let mut error_message: Option<String> = None;
    let mut game: Option<Game> = None;
    let mut users: Option<Vec<User>> = None;

    let conn = database::establish_connection();

    match database::games::load_game_by_code(&conn, &game_code) {
        Ok(g) => { game = Some(g) }
        Err(_) => { error_message = Some(format!("Could not find game code: {}", &game_code)) }
    };

    if let Some(game) = game {
        match database::users::load_users_by_game_id(&conn, game.id) {
            Ok(u) => { users = Some(u) }
            Err(e) => { error_message = Some(format!("Failed to load users: {}", e)) }
        };

        if let Some(users) = users {
            let title = format!("Availability for game {} ({})", game.name, game.code);
            let mut monday = Day::new();
            let mut tuesday = Day::new();
            let mut wednesday = Day::new();
            let mut thursday = Day::new();
            let mut friday = Day::new();
            let mut saturday = Day::new();
            let mut sunday = Day::new();

            for user in users {
                if user.monday {
                    monday.players.push(user.name.clone());
                }
                if user.tuesday {
                    tuesday.players.push(user.name.clone());
                }
                if user.wednesday {
                    wednesday.players.push(user.name.clone());
                }
                if user.thursday {
                    thursday.players.push(user.name.clone());
                }
                if user.friday {
                    friday.players.push(user.name.clone());
                }
                if user.saturday {
                    saturday.players.push(user.name.clone());
                }
                if user.sunday {
                    sunday.players.push(user.name.clone());
                }
            }
            let description = format!(
                "Monday: {}\n
                Tuesday: {}\n
                Wednesday: {}\n
                Thursday: {}\n
                Friday: {}\n
                Saturday: {}\n
                Sunday: {}\n",
                monday,
                tuesday,
                wednesday,
                thursday,
                friday,
                saturday,
                sunday,
            );

            messages::send_message(ctx, title, description).await?;
        }

    }

    if let Some(error_message) = error_message {
        messages::send_error_message(ctx, error_message).await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let title = "Here is a quick rundown of the things you can do:".to_string();
    let description = "`/new_game <game name>`  Create a game and give it a name\n\n\
    `/games`  List all games\n\n\
    `/delete_game <game code>`  Delete the game with the provided game code\n\n\
    `/register_for_game <game code> <multiple data>`  Register for a game, select the days you can do and set at true.\n\n\
    `/availability <game code>`  Show what days people are available for the specified game code.\n\n\
    ".to_string();

    messages::send_message(ctx, title, description).await?;

    Ok(())
}

fn available(day: bool) -> String {
    if day {
        "🟢".to_string()
    } else {
        "🔴".to_string()
    }
}

struct Day {
    pub players: Vec<String>
}

impl Day {
    fn new() -> Day {
        Day { players: Vec::new() }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output_string = "".to_string();
        for player in &self.players {
            output_string.push_str(&format!("{}, ", player))
        }
        write!(f, "{}", output_string)
    }
}
