use chrono::{TimeDelta, Utc};
use rand::Rng;
use serenity::all::{Context, GuildId, Message};
use sqlx::{Database, Pool};

use crate::{Levels, LevelsGuildManager, LevelsManager, LevelsRoleManager, Result};

impl Levels {
    pub async fn message<
        Db: Database,
        GuildManager: LevelsGuildManager<Db>,
        Manager: LevelsManager<Db>,
        RoleManager: LevelsRoleManager<Db>,
    >(
        ctx: &Context,
        message: &Message,
        pool: &Pool<Db>,
    ) -> Result<()> {
        let guild_id = match message.guild_id {
            Some(id) => id,
            None => return Ok(()),
        };

        let guild_row = GuildManager::get(pool, message.guild_id.unwrap())
            .await
            .unwrap()
            .unwrap();

        if guild_row
            .xp_blocked_channels()
            .contains(&message.channel_id)
        {
            return Ok(());
        }

        let row = Manager::get(pool, message.author.id)
            .await
            .unwrap()
            .unwrap();

        if row.last_xp >= (Utc::now().naive_utc() - TimeDelta::minutes(1)) {
            return Ok(());
        }

        let mut level = 0;
        let rand_xp = rand::rng().random_range(15..25);
        let total_xp = row.total_xp + rand_xp;

        let mut xp_for_next_level = 100;
        let mut current_total_xp = 0;
        while total_xp >= current_total_xp + xp_for_next_level {
            current_total_xp += xp_for_next_level;
            level += 1;
            xp_for_next_level = 5 * (level * level) + 50 * level + 100;
        }

        let xp = total_xp - current_total_xp;

        Manager::update(pool, row.id as u64, xp, total_xp, level)
            .await
            .unwrap();

        update_member_roles::<Db, RoleManager>(ctx, message, pool, guild_id, level).await;

        Ok(())
    }
}
async fn update_member_roles<Db: Database, Manager: LevelsRoleManager<Db>>(
    ctx: &Context,
    message: &Message,
    pool: &Pool<Db>,
    guild_id: GuildId,
    level: i32,
) {
    let rows = Manager::get(pool, guild_id, level).await.unwrap();

    let highest_role = rows
        .into_iter()
        .filter(|row| row.level <= level)
        .max_by_key(|row| row.level)
        .unwrap();

    ctx.http
        .add_member_role(guild_id, message.author.id, highest_role.role_id(), None)
        .await
        .unwrap();
}
