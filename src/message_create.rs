use chrono::{TimeDelta, Utc};
use serenity::all::Message;
use sqlx::{Database, Pool};

use crate::LevelsManager;

use super::LevelsRow;

pub async fn message_create<Db: Database, Manager: LevelsManager<Db>>(
    message: &Message,
    pool: &Pool<Db>,
) -> Option<i32> {
    message.guild_id?;

    let mut row = Manager::full_row(pool, message.author.id)
        .await
        .unwrap()
        .unwrap_or_default();

    let xp_cooldown = row.last_xp() + TimeDelta::minutes(1);

    if xp_cooldown > Utc::now().naive_utc() {
        return None;
    }

    let new_level = row.new_message();

    Manager::save(pool, row).await.unwrap();

    new_level
}

// GamblingTable::add_coins(pool, message.author.id, (row.level * 1000) as i64)
//     .await
//     .unwrap();
