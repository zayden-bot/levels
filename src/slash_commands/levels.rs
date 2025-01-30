use futures::{stream, StreamExt};
use serenity::all::{
    CommandInteraction, Context, CreateButton, CreateCommand, CreateEmbed, CreateEmbedFooter,
    EditInteractionResponse, ResolvedOption,
};
use sqlx::{Database, Pool};

use crate::{Levels, LevelsManager, Result};

const PAGE_NUMBER: i64 = 1;

impl Levels {
    pub async fn slash_command<Db: Database, Manager: LevelsManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        _options: Vec<ResolvedOption<'_>>,
        pool: &Pool<Db>,
    ) -> Result<()> {
        interaction.defer(&ctx).await.unwrap();

        let rows = Manager::get_users(pool, PAGE_NUMBER, 10).await.unwrap();

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

        let embed = CreateEmbed::new()
            .title("Leaderboard")
            .fields(fields)
            .footer(CreateEmbedFooter::new(format!("Page {}", PAGE_NUMBER)));

        interaction
            .edit_response(
                &ctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .button(CreateButton::new("levels_previous").label("<"))
                    .button(CreateButton::new("levels_user").emoji('🎯'))
                    .button(CreateButton::new("levels_next").label(">")),
            )
            .await
            .unwrap();

        Ok(())
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("levels").description("Get the leaderboard")
    }
}
