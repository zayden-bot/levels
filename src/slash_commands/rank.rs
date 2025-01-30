use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, EditInteractionResponse, ResolvedOption, ResolvedValue,
};
use sqlx::{Database, Pool};
use zayden_core::parse_options;

use crate::{LevelsManager, Result};

use super::Rank;

impl Rank {
    pub async fn slash_command<Db: Database, Manager: LevelsManager<Db>>(
        ctx: &Context,
        interaction: &CommandInteraction,
        options: Vec<ResolvedOption<'_>>,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let mut options = parse_options(options);

        match options.remove("ephemeral") {
            Some(ResolvedValue::Boolean(true)) => interaction.defer_ephemeral(&ctx).await.unwrap(),
            _ => interaction.defer(&ctx).await.unwrap(),
        }

        let user = match options.remove("user") {
            Some(ResolvedValue::User(user, _)) => user,
            _ => &interaction.user,
        };

        let row = Manager::get(pool, user.id).await.unwrap().unwrap();

        let xp_for_next_level = 5 * (row.level * row.level) + 50 * row.level + 100;
        let user_rank = Manager::get_rank(pool, user.id).await.unwrap().unwrap();

        let embed = CreateEmbed::new()
            .title(format!("XP stats for {}", user.name))
            .description(format!(
                "Rank: #{}\nLevel: {}\nXP: {}/{} ({}%)",
                user_rank,
                row.level,
                row.xp,
                xp_for_next_level,
                (row.xp as f32 / xp_for_next_level as f32 * 100.0).round()
            ));

        interaction
            .edit_response(ctx, EditInteractionResponse::new().embed(embed))
            .await
            .unwrap();

        Ok(())
    }

    pub fn register() -> CreateCommand {
        CreateCommand::new("rank")
            .description("Get your rank or another member's rank")
            .add_option(CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "The user to get the xp of",
            ))
            .add_option(CreateCommandOption::new(
                CommandOptionType::Boolean,
                "ephemeral",
                "Whether the response should be ephemeral",
            ))
    }
}
