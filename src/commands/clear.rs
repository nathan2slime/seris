use chrono::prelude::Utc;

use serenity::{all::MessageId, futures::StreamExt};

use crate::types::{Context, Error};

#[poise::command(
    slash_command,
    description_localized("pt-BR", "Limpa mensagens no canal")
)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    let channel = ctx.channel_id();
    let author = ctx.author();
    let guild_id = ctx.guild_id();
    if guild_id.is_some() {
        let http = ctx.http();

        let guild = http.get_guild(guild_id.unwrap()).await;

        match guild {
            Ok(g) => {
                if g.owner_id == author.id {
                    let messages: Vec<_> = channel
                        .messages_iter(&ctx)
                        .take(40)
                        .boxed()
                        .collect::<Vec<_>>()
                        .await;

                    let message_ids: Vec<MessageId> = messages
                        .into_iter()
                        .filter_map(|msgn| match msgn {
                            Ok(msg) => Some(msg.id),
                            Err(_) => None,
                        })
                        .collect();

                    channel.delete_messages(&ctx, message_ids).await?;

                    ctx.say(format!(
                        "Houve uma limpeza de mensagens — <t:{}>.",
                        Utc::now().timestamp()
                    ))
                    .await?;
                } else {
                    ctx.say("Sinto muito. Só o dono do servidor pode apagar mensagens")
                        .await?;
                }
            }
            Err(_) => {}
        };
    } else {
        ctx.say("Sinto muito — isso só pode ser executado em um servidor")
            .await?;
    }

    Ok(())
}
