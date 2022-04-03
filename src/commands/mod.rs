use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::{
    game_code,
    messages,
    arguments,
    models::{Game, User},
    database
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
async fn register_for_game(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let (game_code, days_playable) = arguments::parse_day_registration(args)?;
    let player = &msg.author.name;

    let mut game: Option<Game> = None;
    let mut user: Option<User> = None;
    let mut error_message: Option<String> = None;

    let conn = database::establish_connection();
    match database::games::load_game_by_code(&conn, &game_code) {
        Ok(g) => { game = Some(g) }
        Err(e) => { error_message = Some(e.to_string()) }
    };

    if game.is_some() {
        match database::users::create_user(&conn, player, game.unwrap().id, days_playable) {
            Ok(u) => { user = Some(u) }
            Err(e) => { error_message = Some(e.to_string()) }
        };
    };

    if error_message.is_some() {
        messages::send_error(ctx, msg, error_message.unwrap()).await?;
    }


    let title = "This is a title".to_string();
    let description = "🔴 Monday\n 🟠 Tuesday\n 🟡 Wednesday\n 🟢 Thursday\n 🔵 Friday\n 🟣 Saturday\n 🟤 Sunday\n".to_string();

    messages::send(ctx, msg, title, description).await?;

    Ok(())
}
