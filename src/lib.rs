use async_trait::async_trait;
use chrono::NaiveDateTime;
use serenity::all::{ChannelId, GuildId, RoleId, UserId};
use sqlx::{Database, Pool};

pub mod component;
pub mod error;
pub mod message_command;
pub mod slash_commands;

pub use error::Error;
use error::Result;

pub struct Levels;

#[async_trait]
pub trait LevelsGuildManager<Db: Database> {
    async fn get(
        pool: &Pool<Db>,
        guild_id: impl Into<GuildId> + Send,
    ) -> sqlx::Result<Option<LevelsGuildRow>>;
}

pub struct LevelsGuildRow {
    pub id: i64,
    pub xp_blocked_channels: Vec<i64>,
}

impl LevelsGuildRow {
    pub fn guild_id(&self) -> GuildId {
        GuildId::new(self.id as u64)
    }

    pub fn xp_blocked_channels(&self) -> Vec<ChannelId> {
        self.xp_blocked_channels
            .iter()
            .copied()
            .map(|id| ChannelId::new(id as u64))
            .collect()
    }
}

#[async_trait]
pub trait LevelsManager<Db: Database> {
    async fn get(pool: &Pool<Db>, id: impl Into<UserId> + Send) -> sqlx::Result<Option<LevelsRow>>;

    async fn get_users(pool: &Pool<Db>, page: i64, limit: i64) -> sqlx::Result<Vec<LevelsRow>>;

    async fn get_rank(pool: &Pool<Db>, id: impl Into<UserId> + Send) -> sqlx::Result<Option<i64>>;

    async fn update(
        pool: &Pool<Db>,
        id: impl Into<UserId> + Send,
        xp: i32,
        total_xp: i32,
        level: i32,
    ) -> sqlx::Result<()>;
}

pub struct LevelsRow {
    pub id: i64,
    pub xp: i32,
    pub level: i32,
    pub total_xp: i32,
    pub message_count: i32,
    pub last_xp: NaiveDateTime,
}

impl LevelsRow {
    pub fn user_id(&self) -> UserId {
        UserId::new(self.id as u64)
    }
}

#[async_trait]
pub trait LevelsRoleManager<Db: Database> {
    async fn get(
        pool: &Pool<Db>,
        guild_id: impl Into<GuildId> + Send,
        level: i32,
    ) -> sqlx::Result<Vec<LevelRoleRow>>;
}

pub struct LevelRoleRow {
    pub id: i64,
    pub guild_id: i64,
    pub level: i32,
}

impl LevelRoleRow {
    pub fn role_id(&self) -> RoleId {
        RoleId::new(self.id as u64)
    }

    pub fn guild_id(&self) -> GuildId {
        GuildId::new(self.guild_id as u64)
    }
}
