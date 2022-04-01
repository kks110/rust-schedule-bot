use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::{game_id, messages, arguments, models};
use serde_json;
use crate::models::GameData;

#[command]
pub async fn new_game(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let game_name =  args.single::<String>()?;
    let game_id = game_id::generate();

    let title = format!("Game Created: {}", game_name);
    let description = format!("Please log your days using the game ID: **{}**", game_id);

    messages::send(ctx, msg, title, description).await?;

    Ok(())
}

#[command]
async fn register_for_game(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let days_playable = arguments::parse_day_registration(args)?;
    let player = &msg.author.name;
    // for day in days_playable.weekdays {
    //     println!("{}", day)
    // }

    let game_data = GameData::new(days_playable.game_id, player.to_owned(), days_playable.weekdays);

    std::fs::write(
        "game_data.json",
        serde_json::to_string_pretty(&game_data).unwrap(),
    )?;

    let title = "This is a title".to_string();
    let description = "🔴 Monday\n 🟠 Tuesday\n 🟡 Wednesday\n 🟢 Thursday\n 🔵 Friday\n 🟣 Saturday\n 🟤 Sunday\n".to_string();

    messages::send(ctx, msg, title, description).await?;

    Ok(())
}
