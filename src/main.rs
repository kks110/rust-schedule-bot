mod game_id;
mod messages;
mod commands;
mod arguments;
mod models;

use std::env;

use serenity::{
    async_trait,
    framework::{
        standard::macros::group,
        StandardFramework,
    },
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use tracing::info;
use serenity::model::prelude::*;
use serenity::framework::standard::{Args, CommandResult, macros::command};
use dotenv::dotenv;
use crate::commands::{
    NEW_GAME_COMMAND,
    REGISTER_FOR_GAME_COMMAND
};
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        let _ = &__arg1;
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        let _ = &__arg1;
        info!("Resumed");
    }
}

#[group]
#[commands(register_for_game, new_game)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c
        .prefix("!")
        .delimiters(vec![" "]))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
