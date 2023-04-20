use std::fmt::{Display, Formatter};
use crate::{
    game_code,
    messages,
    models::{Game, User},
    database,
    Context,
    Error
};

/// Create a new game for people to join
#[poise::command(slash_command)]
pub async fn new_game(
    ctx: Context<'_>,
    #[description = "Game name"]
    game_name: String,
) -> Result<(), Error> {
    let game_code = game_code::generate();

    let mut error_message: Option<String> = None;

    let conn = database::establish_connection();
    match database::games::create_game(&conn, &game_code, &game_name, 10) {
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

/// List all games available
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
                    description.push_str(&format!("**{}**\n Code: **{}**\n {}/{} players\n\n", game.name, game.code, user_count, game.user_count));
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

/// Delete a game that is no longer needed
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

/// Register for a game, entering your free days
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
    #[description = "Saturday"]
    #[lazy]
    saturday: Option<bool>,
    #[description = "Sunday"]
    #[lazy]
    sunday: Option<bool>,
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
    if let Some(sat) = saturday {
        if sat {
            days_playable.push("saturday".to_string())
        }
    }
    if let Some(sun) = sunday {
        if sun {
            days_playable.push("sunday".to_string())
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

/// See who is free and when for a specified game
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
            let mut monday = Day::new("Monday".to_string());
            let mut tuesday = Day::new("Tuesday".to_string());
            let mut wednesday = Day::new("Wednesday".to_string());
            let mut thursday = Day::new("Thursday".to_string());
            let mut friday = Day::new("Friday".to_string());
            let mut saturday = Day::new("Saturday".to_string());
            let mut sunday = Day::new("Sunday".to_string());

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
                "{}{}{}{}{}{}{}",
                monday,
                tuesday,
                wednesday,
                thursday,
                friday,
                saturday,
                sunday
            );

            messages::send_message(ctx, title, description).await?;
        }

    }

    if let Some(error_message) = error_message {
        messages::send_error_message(ctx, error_message).await?;
    }

    Ok(())
}

/// Show the help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Type /help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
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
    pub players: Vec<String>,
    pub name: String
}

impl Day {
    fn new(name: String) -> Day {
        Day { players: Vec::new(), name }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output_string = "".to_string();

        if !self.players.is_empty() {
            output_string.push_str(&format!("**{}** ({} player(s))\n", self.name.to_string(), self.players.len()));

            for player in &self.players {
                output_string.push_str(&format!("{}, ", player))
            }

            output_string.push_str(&format!("\n\n"));
        }
        write!(f, "{}", output_string)
    }
}
