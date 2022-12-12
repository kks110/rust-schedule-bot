use crate::Context;
use crate::Error;

pub async fn send_message<
    D: ToString,
>(ctx: Context<'_>, title: D, description: D) -> Result<(), Error> {
    ctx.send(|b| {
        b.embed(|b| {
            b.colour(0xcc8800)
                .title(title)
                .description(description)
        })
    }).await?;

    Ok(())
}

// pub async fn send_message_with_fields<
//     D: ToString,
//     T,
//     U,
//     It
// >(ctx: Context<'_>, title: D, fields: It) -> Result<(), Error>
//     where
//         It: IntoIterator<Item = (T, U, bool)>,
//         T: ToString,
//         U: ToString,
// {
//     ctx.send(|b| {
//         b.embed(|b| {
//             b.colour(0xcc8800)
//                 .title(title)
//                 .fields(fields)
//         })
//     }).await?;
//
//     Ok(())
// }

pub async fn send_error_message<
    D: ToString,
>(ctx: Context<'_>, title: D) -> Result<(), Error> {
    ctx.send(|b| {
        b.embed(|b| {
            b.colour(0xcc0000)
                .title(title)
        })
    }).await?;

    Ok(())
}
