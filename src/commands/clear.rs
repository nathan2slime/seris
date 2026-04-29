use chrono::prelude::Utc;

use serenity::{all::MessageId, futures::StreamExt};

use crate::types::{Context, Error};

/// Deletes recent messages in a guild channel for the owner.
#[poise::command(
    slash_command,
    description_localized("pt-BR", "Limpa mensagens do canal, se puder")
)]
pub async fn clear(ctx: Context<'_>) -> Result<(), Error> {
    ctx.data()
        .database
        .record_command_usage_best_effort(ctx.author().id, "clear");

    ctx.defer().await?;

    let channel = ctx.channel_id();
    let author = ctx.author();
    let guild_id = ctx.guild_id();
    if let Some(guild_id) = guild_id {
        let http = ctx.http();

        let guild = http.get_guild(guild_id).await;

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
                            Err(err) => {
                                log::error!(
                                    "failed to fetch message for deletion in {guild_id}: {err}"
                                );
                                None
                            }
                        })
                        .collect();

                    channel.delete_messages(&ctx, message_ids).await?;

                    ctx.say(format!(
                        "P-pronto... limpei as mensagens — <t:{}>.",
                        Utc::now().timestamp()
                    ))
                    .await?;
                } else {
                    ctx.say("D-desculpe... só o dono do servidor pode apagar mensagens")
                        .await?;
                }
            }
            Err(err) => {
                log::error!("failed to fetch guild {guild_id}: {err}");
                ctx.say("D-desculpe... não consegui confirmar as permissões do servidor")
                    .await?;
            }
        };
    } else {
        ctx.say("D-desculpe... isso só pode ser executado em um servidor")
            .await?;
    }

    Ok(())
}
