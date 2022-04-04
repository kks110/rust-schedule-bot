use std::fmt::format;
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

        let title = format!("{}'s available daya", u.name);
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

pub async fn view_availability(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let game_code = arguments::parse_string(args)?;
    let conn = database::establish_connection();
    let game = database::games::load_game_by_code(&conn, &game_code)?;
    let users: Vec<User> = database::users::load_users_by_game_id(&conn, game.id)?;

    let title = format!("Availability for game {} ({})", game.name, game.id);


}

fn available(day: bool) -> String {
    if day {
        "🟢".to_string()
    } else {
        "🔴".to_string()
    }
}
