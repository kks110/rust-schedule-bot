use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, format, Formatter};
use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::{
    game_code,
    messages,
    arguments,
    models::{Game, User},
    database,
    validation
};

#[command]
pub async fn new_game(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let game_name =  args.single::<String>()?;
    let game_code = game_code::generate();

    let mut error_message: Option<String> = None;

    let conn = database::establish_connection();
    match database::games::create_game(&conn, &game_code, &game_name, 5) {
        Ok(_) => {}
        Err(e) => { error_message = Some(e.to_string()) }
    };

    if error_message.is_some() {
        messages::send_error(ctx, msg, error_message.unwrap()).await?;
    }

    let title = format!("Game Created: {}", game_name);
    let description = format!("Please log your days using the game ID: **{}**", game_code);

    messages::send(ctx, msg, title, description).await?;

    Ok(())
}

#[command]
pub async fn games(ctx: &Context, msg: &Message) -> CommandResult {
    let mut error_message: Option<String> = None;
    let mut games: Option<Vec<Game>> = None;

    let conn = database::establish_connection();
    match database::games::load_games(&conn) {
        Ok(g) => { games = Some(g) }
        Err(e) => { error_message = Some(e.to_string()) }
    };

    let mut description = "".to_string();
    let title = format!("Game List:");

    if games.is_some() {
        for game in games.unwrap() {
            match database::users::load_user_count_by_game_id(&conn, game.id) {
                Ok(user_count) => {
                    description.push_str(&format!("{} ({}) - {}/{} players registered\n", game.name, game.code, user_count, game.user_count));
                }
                Err(e) => { error_message = Some(e.to_string()) }
            }
        }
    }

    if error_message.is_some() {
        messages::send_error(ctx, msg, error_message.unwrap()).await?;
    }

    messages::send(ctx, msg, title, description).await?;

    Ok(())
}

#[command]
pub async fn delete_game(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let game_code = arguments::parse_string(args)?;

    let mut error_message: Option<String> = None;

    let conn = database::establish_connection();

    match database::games::delete_game(&conn, &game_code) {
        Ok(_) => {}
        Err(e) => { error_message = Some(format!("Error deleting game: {}", e)) }
    };


    if error_message.is_some() {
        messages::send_error(ctx, msg, error_message.unwrap()).await?;
    }

    messages::send(ctx, msg, "Game delete successfully", "").await?;

    Ok(())
}

#[command]
pub async fn register_for_game(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (game_code, days_playable) = arguments::parse_day_registration(args)?;
    let player = &msg.author.name;

    let mut game: Option<Game> = None;
    let mut user: Option<User> = None;
    let mut error_message: Option<String> = None;

    match validation::validate_week_days(&days_playable) {
        Ok(_) => {}
        Err(e) => {error_message = Some(e)}
    };

    let conn = database::establish_connection();
    match database::games::load_game_by_code(&conn, &game_code) {
        Ok(g) => { game = Some(g) }
        Err(_) => { error_message = Some(format!("Could not find game code: {}", &game_code)) }
    };

    if game.is_some() {
        match database::users::update_or_create(&conn, player, game.unwrap().id, days_playable) {
            Ok(u) => { user = Some(u) }
            Err(e) => { error_message = Some(format!("Whilst registering for the game the following error occurred: {}", e.to_string())) }
        };
    };

    if error_message.is_some() {
        messages::send_error(ctx, msg, error_message.unwrap()).await?;
    }

    if user.is_some() {
        let u = user.unwrap();

        let title = format!("{}'s available days", u.name);
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

        messages::send(ctx, msg, title, description).await?;
    }

    Ok(())
}

#[command]
pub async fn availability(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut error_message: Option<String> = None;
    let mut game: Option<Game> = None;
    let mut users: Option<Vec<User>> = None;

    let game_code = arguments::parse_string(args)?;
    let conn = database::establish_connection();


    match database::games::load_game_by_code(&conn, &game_code) {
        Ok(g) => { game = Some(g) }
        Err(_) => { error_message = Some(format!("Could not find game code: {}", &game_code)) }
    };

    if game.is_some() {
        let safe_game = game.unwrap();
        match database::users::load_users_by_game_id(&conn, safe_game.id) {
            Ok(u) => { users = Some(u) }
            Err(e) => { error_message = Some(format!("Failed to load users: {}", e)) }
        };

        if users.is_some() {
            let title = format!("Availability for game {} ({})", safe_game.name, safe_game.code);
            let mut days_and_players: HashMap<&str, Vec<&String>> = HashMap::new();

            let mut monday = Day::new();
            let mut tuesday = Day::new();
            let mut wednesday = Day::new();
            let mut thursday = Day::new();
            let mut friday = Day::new();
            let mut saturday = Day::new();
            let mut sunday = Day::new();

            for user in users.unwrap() {
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

            messages::send(ctx, msg, title, description).await?;
        }

    }

    if error_message.is_some() {
        messages::send_error(ctx, msg, error_message.unwrap()).await?;
    }



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
