mod game_code;
mod messages;
mod commands;
mod models;
mod database;
mod schema;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use poise::serenity_prelude as serenity;
use crate::database::run_migrations;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    run_migrations();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::new_game(),
                commands::availability(),
                commands::games(),
                commands::delete_game(),
                commands::help(),
                commands::register_for_game(),
            ],
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("Executing command {}...", ctx.command().qualified_name);
                })
            },
            /// This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    println!("Executed command {}!", ctx.command().qualified_name);
                })
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .token(std::env::var("SCHEDULE_DISCORD_TOKEN").expect("missing SCHEDULE_DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::serenity_prelude::GuildId(844882826930421800)
                    .set_application_commands(ctx, |b| {
                        *b = poise::builtins::create_application_commands(
                            &framework.options().commands,
                        );
                        b
                    })
                    .await
                    .unwrap();
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
