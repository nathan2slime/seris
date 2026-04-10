//! Shared EPIC session state and component handling.

use poise::{serenity_prelude as serenity, BoxFuture};
use serenity::{
    all::{ButtonStyle, ComponentInteraction, FullEvent, Interaction, UserId},
    builder::{
        CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
};

use crate::{
    embeds,
    services::epic::EpicImage,
    types::{Data, Error},
};

const ACTION_PREFIX: &str = "epic:";

/// A live EPIC gallery session tied to a single Discord message.
#[derive(Clone, Debug)]
pub struct EpicSession {
    owner_id: UserId,
    kind: String,
    images: Vec<EpicImage>,
    current_index: usize,
}

impl EpicSession {
    /// Creates a new gallery session.
    pub fn new(owner_id: UserId, kind: String, images: Vec<EpicImage>) -> Self {
        Self {
            owner_id,
            kind,
            images,
            current_index: 0,
        }
    }

    /// Returns the current image.
    pub fn current_image(&self) -> &EpicImage {
        &self.images[self.current_index]
    }

    /// Returns the current zero-based page index.
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Returns the total number of pages.
    pub fn total_pages(&self) -> usize {
        self.images.len()
    }

    /// Returns the active kind label.
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Builds the message components for the current page.
    pub fn components(&self) -> Vec<CreateActionRow> {
        let is_first = self.current_index == 0;
        let is_last = self.current_index + 1 >= self.images.len();

        vec![CreateActionRow::Buttons(vec![
            CreateButton::new(format!("{ACTION_PREFIX}first"))
                .label("Primeira")
                .style(ButtonStyle::Secondary)
                .disabled(is_first),
            CreateButton::new(format!("{ACTION_PREFIX}prev"))
                .label("Anterior")
                .style(ButtonStyle::Secondary)
                .disabled(is_first),
            CreateButton::new(format!("{ACTION_PREFIX}next"))
                .label("Próxima")
                .style(ButtonStyle::Secondary)
                .disabled(is_last),
            CreateButton::new(format!("{ACTION_PREFIX}last"))
                .label("Última")
                .style(ButtonStyle::Secondary)
                .disabled(is_last),
        ])]
    }

    /// Builds the current embed.
    pub fn embed(&self) -> CreateEmbed {
        embeds::epic::epic(
            self.current_image(),
            self.current_index,
            self.images.len(),
            &self.kind,
        )
    }

    /// Applies a navigation action to the session.
    pub fn navigate(&mut self, action: EpicAction) {
        match action {
            EpicAction::First => self.current_index = 0,
            EpicAction::Prev => {
                self.current_index = self.current_index.saturating_sub(1);
            }
            EpicAction::Next => {
                self.current_index =
                    (self.current_index + 1).min(self.images.len().saturating_sub(1));
            }
            EpicAction::Last => {
                self.current_index = self.images.len().saturating_sub(1);
            }
        }
    }
}

/// EPIC navigation actions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpicAction {
    /// Jump to the first page.
    First,
    /// Go back one page.
    Prev,
    /// Advance one page.
    Next,
    /// Jump to the final page.
    Last,
}

fn parse_action(custom_id: &str) -> Option<EpicAction> {
    match custom_id.strip_prefix(ACTION_PREFIX)? {
        "first" => Some(EpicAction::First),
        "prev" => Some(EpicAction::Prev),
        "next" => Some(EpicAction::Next),
        "last" => Some(EpicAction::Last),
        _ => None,
    }
}

async fn handle_component_interaction(
    ctx: &serenity::Context,
    data: &Data,
    interaction: &ComponentInteraction,
) -> Result<(), Error> {
    let Some(action) = parse_action(&interaction.data.custom_id) else {
        return Ok(());
    };

    let message_id = interaction.message.id.get();
    enum Reject {
        Missing,
        Forbidden,
    }

    let render = match data.epic_sessions.get_mut(&message_id) {
        None => Err(Reject::Missing),
        Some(mut session) => {
            if session.owner_id != interaction.user.id {
                Err(Reject::Forbidden)
            } else {
                session.navigate(action);
                Ok((session.embed(), session.components()))
            }
        }
    };

    let (embed, components) = match render {
        Ok(render) => render,
        Err(Reject::Missing) => {
            interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("E-eu perdi essa galeria... tente abrir o comando de novo.")
                            .ephemeral(true),
                    ),
                )
                .await?;
            return Ok(());
        }
        Err(Reject::Forbidden) => {
            interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("D-desculpe... só quem abriu o comando pode navegar.")
                            .ephemeral(true),
                    ),
                )
                .await?;
            return Ok(());
        }
    };

    interaction
        .create_response(
            ctx,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .embeds(vec![embed])
                    .components(components),
            ),
        )
        .await?;

    Ok(())
}

/// Framework event handler that routes EPIC button clicks.
pub fn framework_event_handler<'a>(
    ctx: &'a serenity::Context,
    event: &'a FullEvent,
    _framework: poise::FrameworkContext<'a, Data, Error>,
    data: &'a Data,
) -> BoxFuture<'a, Result<(), Error>> {
    Box::pin(async move {
        if let FullEvent::InteractionCreate {
            interaction: Interaction::Component(interaction),
        } = event
        {
            handle_component_interaction(ctx, data, interaction).await?;
        }

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_action, EpicAction, EpicSession};
    use crate::services::epic::EpicImage;
    use serenity::all::UserId;

    #[test]
    fn parses_actions() {
        assert_eq!(parse_action("epic:first"), Some(EpicAction::First));
        assert_eq!(parse_action("epic:prev"), Some(EpicAction::Prev));
        assert_eq!(parse_action("epic:next"), Some(EpicAction::Next));
        assert_eq!(parse_action("epic:last"), Some(EpicAction::Last));
        assert_eq!(parse_action("noop"), None);
    }

    #[test]
    fn navigates_pages() {
        let mut session = EpicSession::new(
            UserId::new(1),
            "natural".to_string(),
            vec![
                EpicImage {
                    image: "1".to_string(),
                    caption: "one".to_string(),
                    date: "2026-04-09 00:00:00".to_string(),
                    url: "https://example.com/1".to_string(),
                },
                EpicImage {
                    image: "2".to_string(),
                    caption: "two".to_string(),
                    date: "2026-04-09 00:00:01".to_string(),
                    url: "https://example.com/2".to_string(),
                },
            ],
        );

        assert_eq!(session.current_index(), 0);
        session.navigate(EpicAction::Next);
        assert_eq!(session.current_index(), 1);
        session.navigate(EpicAction::Prev);
        assert_eq!(session.current_index(), 0);
        session.navigate(EpicAction::Last);
        assert_eq!(session.current_index(), 1);
    }
}
