use std::fmt::Display;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

pub async fn send<T: Display>(
    ctx: &Context,
    msg: &Message,
    title: T,
    description: T,
) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.colour(0x9003fc);
            e.title(title);
            e.description(description);

            e
        })
    }).await?;
    Ok(())
}

pub async fn send_error<T: Display>(ctx: &Context, msg: &Message, error: T) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e
                .colour(0xff5733)
                .title(format!("❌ An error as occurred: {}", error))
        })
    }).await?;
    Ok(())
}