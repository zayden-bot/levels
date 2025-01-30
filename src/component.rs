use futures::{stream, StreamExt};
use serenity::all::{
    ComponentInteraction, Context, CreateEmbed, CreateEmbedFooter, EditInteractionResponse,
    MessageInteractionMetadata,
};
use sqlx::{Database, Pool};

use crate::{Error, Levels, LevelsManager, Result};

const LIMIT: i64 = 10;

impl Levels {
    pub async fn component<Db: Database, Manager: LevelsManager<Db>>(
        ctx: &Context,
        interaction: &ComponentInteraction,
        pool: &Pool<Db>,
    ) -> Result<()> {
        interaction.defer(ctx).await.unwrap();

        if let Some(MessageInteractionMetadata::Component(metadata)) =
            interaction.message.interaction_metadata.as_deref()
        {
            if metadata.user != interaction.user {
                return Err(Error::NotInteractionAuthor);
            }
        }

        let mut page_number = interaction
            .message
            .embeds
            .first()
            .unwrap()
            .footer
            .as_ref()
            .unwrap()
            .text
            .strip_prefix("Page ")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let mut embed = CreateEmbed::new().title("Leaderboard");

        match interaction.data.custom_id.as_str() {
            "levels_previous" => {
                page_number = (page_number - 1).max(1);
            }
            "levels_user" => {
                let row_number = Manager::get_rank(pool, interaction.user.id)
                    .await
                    .unwrap()
                    .unwrap()
                    - 1;

                page_number = row_number / LIMIT + 1;
            }
            "levels_next" => {
                page_number += 1;
            }
            _ => unreachable!(),
        };

        let rows = Manager::get_users(pool, page_number, LIMIT).await.unwrap();

        let fields = stream::iter(rows)
            .then(|row| async move {
                let user = row.user_id().to_user(&ctx).await.unwrap();

                (
                    user.global_name.unwrap_or(user.name),
                    format!(
                        "Messages: {} | Total XP: {} | Level: {}",
                        row.message_count, row.xp, row.level
                    ),
                    false,
                )
            })
            .collect::<Vec<_>>()
            .await;

        embed = embed
            .footer(CreateEmbedFooter::new(format!("Page {}", page_number)))
            .fields(fields);

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }
}
